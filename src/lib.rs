pub mod error;
pub mod io;
pub mod target;

use error::MakuError;
use io::{load_shader, resolve_resource_path, IoProject};
use three_d::{Object, SquareMatrix};

/// Represents different types of filters that can be applied to an image
pub enum Filter {
    /// An image filter, containing a texture reference
    Image(three_d::Texture2DRef),
    /// A shader filter, containing a program
    Shader(three_d::Program),
}

/// Main structure for the Maku image processing system
pub struct Maku {
    /// Input texture for processing
    input: three_d::Texture2D,
    /// Output texture after processing
    output: three_d::Texture2D,
    /// Camera for rendering
    camera: three_d::Camera,
    /// List of filters to be applied
    filters: Vec<Filter>,
    /// Vertex buffer for rendering a plane
    plane_positions: three_d::VertexBuffer,
    /// Program for copying textures
    copy_program: three_d::Program,
}

impl Maku {
    /// Load a Maku instance from a JSON configuration file
    pub async fn load(
        context: &three_d::Context,
        json_path: std::path::PathBuf,
    ) -> Result<Maku, MakuError> {
        log::debug!("Load json: {:?}", json_path);
        let json = std::fs::read_to_string(json_path.clone())?;
        let project = serde_json::from_str::<IoProject>(&json).map_err(MakuError::from)?;

        // Load resources and create filters
        let mut filters = vec![];
        for filter in project.filters.iter() {
            match filter {
                io::IoFilter::Image { path } => {
                    // Load image filter
                    let path = resolve_resource_path(path, &json_path);
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());
                    filters.push(Filter::Image(three_d::Texture2DRef {
                        texture: image.into(),
                        transformation: three_d::Mat3::identity(),
                    }));
                }
                io::IoFilter::Shader(shader) => {
                    // Load shader filter
                    let (vert, frag) = load_shader(shader, &json_path);
                    let program = three_d::Program::from_source(context, &vert, &frag).unwrap();
                    filters.push(Filter::Shader(program));
                }
            }
        }

        let viewport = three_d::Viewport::new_at_origo(project.width, project.height);
        let mut camera = three_d::Camera::new_2d(viewport);

        camera.disable_tone_and_color_mapping();

        // For copy textures
        let copy_program = three_d::Program::from_source(
            context,
            "
                in vec4 position;
                void main() {
                  gl_Position = position;
                }
            ",
            "
                uniform sampler2D u_texture;
                uniform vec2 u_resolution;
                out vec4 outColor;
                void main() {
                  outColor = texture(u_texture, gl_FragCoord.xy / u_resolution);
                }
            ",
        )
        .unwrap();
        let plane_positions = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(-1.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, -1.0, 0.0),
            ],
        );

        Ok(Maku {
            input: new_texture(context, project.width, project.height),
            output: new_texture(context, project.width, project.height),
            camera,
            filters,
            plane_positions,
            copy_program,
        })
    }

    pub fn width(&self) -> u32 {
        self.camera.viewport().width
    }

    pub fn height(&self) -> u32 {
        self.camera.viewport().height
    }

    /// Render the image with all applied filters
    pub fn render(&mut self, target: &mut target::Target) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        target.clear(three_d::ClearState::default());

        for filter in self.filters.iter() {
            // Apply each filter
            self.output.as_color_target(None).write(|| {
                match filter {
                    Filter::Image(texture) => {
                        // Render image filter
                        let model = three_d::Gm::new(
                            three_d::Rectangle::new(
                                target.context(),
                                three_d::vec2(width * 0.5, height * 0.5),
                                three_d::degrees(0.0),
                                width,
                                height,
                            ),
                            three_d::ColorMaterial {
                                texture: Some(texture.clone()),
                                ..Default::default()
                            },
                        );

                        model.render(&self.camera, &[]);
                    }
                    Filter::Shader(program) => {
                        // Apply shader filter
                        program.use_uniform(
                            "u_resolution",
                            three_d::Vector2 {
                                x: width,
                                y: height,
                            },
                        );
                        program.use_vertex_attribute("position", &self.plane_positions);
                        program.use_texture("u_texture", &self.input);
                        program.draw_arrays(
                            three_d::RenderStates::default(),
                            self.camera.viewport(),
                            self.plane_positions.vertex_count(),
                        );
                    }
                }
                Ok::<(), MakuError>(())
            })?;

            // Copy output to input for next filter
            self.input.as_color_target(None).write(|| {
                self.copy_program.use_uniform(
                    "u_resolution",
                    three_d::Vector2 {
                        x: width,
                        y: height,
                    },
                );
                self.copy_program
                    .use_vertex_attribute("position", &self.plane_positions);
                self.copy_program.use_texture("u_texture", &self.output);
                self.copy_program.draw_arrays(
                    three_d::RenderStates::default(),
                    self.camera.viewport(),
                    self.plane_positions.vertex_count(),
                );
                Ok::<(), MakuError>(())
            })?;
        }

        // Copy final output to the target
        target.write(|| {
            self.copy_program.use_uniform(
                "u_resolution",
                three_d::Vector2 {
                    x: width,
                    y: height,
                },
            );
            self.copy_program
                .use_vertex_attribute("position", &self.plane_positions);
            self.copy_program.use_texture("u_texture", &self.output);
            self.copy_program.draw_arrays(
                three_d::RenderStates::default(),
                self.camera.viewport(),
                self.plane_positions.vertex_count(),
            );
            Ok::<(), MakuError>(())
        })?;

        Ok(())
    }

    /// Render the image with all applied filters and save it to a file
    pub fn render_to_file(
        &mut self,
        context: &three_d::Context,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        // Create a new texture for rendering
        let texture = new_texture(context, self.width(), self.height());
        let mut target = target::Target::Pixels {
            context: context.clone(),
            texture,
        };

        // Render to the target
        self.render(&mut target)?;

        // Save the rendered image to a file
        let pixels = target.pixels();
        image::save_buffer_with_format(
            output_path,
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )?;

        Ok(())
    }
}

/// Create a new empty texture with the specified dimensions
fn new_texture(context: &three_d::Context, width: u32, height: u32) -> three_d::Texture2D {
    three_d::Texture2D::new_empty::<[u8; 4]>(
        context,
        width,
        height,
        three_d::Interpolation::Linear,
        three_d::Interpolation::Linear,
        None,
        three_d::Wrapping::ClampToEdge,
        three_d::Wrapping::ClampToEdge,
    )
}
