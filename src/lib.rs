pub mod error;
pub mod io;

use error::MakuError;
use io::IoProject;
use three_d::{Object, SquareMatrix};

pub enum Filter {
    Image(three_d::Texture2DRef),
    Shader(three_d::Program),
}

pub struct Maku {
    input: three_d::Texture2D,
    output: three_d::Texture2D,
    camera: three_d::Camera,
    filters: Vec<Filter>,
    //
    plane_positions: three_d::VertexBuffer,
    copy_program: three_d::Program,
}

pub enum Target {
    Screen {
        context: three_d::Context,
        width: u32,
        height: u32,
    },
    Pixels {
        context: three_d::Context,
        texture: three_d::Texture2D,
    },
}

impl Target {
    pub fn context(&self) -> &three_d::Context {
        match self {
            Target::Screen { context, .. } => context,
            Target::Pixels { context, .. } => context,
        }
    }
    pub fn clear(&mut self, clear_state: three_d::ClearState) -> &Self {
        match self {
            Target::Screen {
                context,
                width,
                height,
            } => {
                three_d::RenderTarget::screen(context, *width, *height).clear(clear_state);
            }
            Target::Pixels { texture, .. } => {
                texture.as_color_target(None).clear(clear_state);
            }
        }
        self
    }
    pub fn write<E: std::error::Error>(
        &mut self,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            Target::Screen {
                context,
                width,
                height,
            } => {
                three_d::RenderTarget::screen(context, *width, *height).write(render)?;
            }
            Target::Pixels { texture, .. } => {
                texture.as_color_target(None).write(render)?;
            }
        }
        Ok(())
    }
    pub fn pixels(&mut self) -> Vec<u8> {
        match self {
            Target::Pixels { texture, .. } => texture
                .as_color_target(None)
                .read::<[u8; 4]>()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
            Target::Screen { .. } => unreachable!(),
        }
    }
}

impl Maku {
    pub async fn load(
        context: &three_d::Context,
        json_path: std::path::PathBuf,
    ) -> Result<Maku, MakuError> {
        log::debug!("Load json: {:?}", json_path);
        let json = std::fs::read_to_string(json_path)?;
        let project = serde_json::from_str::<IoProject>(&json).map_err(MakuError::from)?;
        Maku::load_project(context, project).await
    }

    pub async fn load_project(
        context: &three_d::Context,
        project: IoProject,
    ) -> Result<Maku, MakuError> {
        // Load resources
        let mut filters = vec![];
        for filter in project.filters.iter() {
            match filter {
                io::IoFilter::Image { path } => {
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());
                    filters.push(Filter::Image(three_d::Texture2DRef {
                        texture: image.into(),
                        transformation: three_d::Mat3::identity(),
                    }));
                }
                io::IoFilter::Shader { fragment, vertex } => {
                    let vert: String = vertex.into();
                    let frag: String = fragment.into();
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

    pub fn render(&mut self, target: &mut Target) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        target.clear(three_d::ClearState::default());

        for filter in self.filters.iter() {
            self.output.as_color_target(None).write(|| {
                match filter {
                    Filter::Image(texture) => {
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

            // Copy output to input
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

        // Copy output to the target
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

    pub fn render_to_file(
        &mut self,
        context: &three_d::Context,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        let texture = new_texture(context, self.width(), self.height());
        let mut target = Target::Pixels {
            context: context.clone(),
            texture,
        };

        self.render(&mut target)?;

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
