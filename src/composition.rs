use crate::error::MakuError;
use crate::io;
use crate::target;

/// Represents different types of filters that can be applied to an image
pub enum Filter {
    /// An composition filter
    Composition(Composition),
    /// An image filter, containing a texture reference
    Image(three_d::Texture2DRef, three_d::VertexBuffer),
    /// A shader filter, containing a program
    Shader(three_d::Program, Vec<(String, f32)>),
}

// Composition
pub struct Composition {
    /// Input texture for processing
    input: three_d::Texture2D,
    /// Output texture after processing
    output: three_d::Texture2D,
    /// Camera for rendering
    camera: three_d::Camera,
    /// List of filters to be applied
    filters: Vec<Filter>,
    /// Program for copying textures
    copy_program: three_d::Program,
}

impl Composition {
    pub async fn load(
        context: &three_d::Context,
        composition: &io::IoComposition,
        parent_dir: &std::path::Path,
    ) -> Result<Self, MakuError> {
        // Load resources and create filters

        let mut filters = vec![];
        for filter in composition.filters.iter() {
            match filter {
                io::IoFilter::Image { path, fit } => {
                    // Load image filter

                    let path = io::resolve_resource_path(parent_dir, path);
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());

                    let (sx, sy) = match fit {
                        io::IoImageFit::Fill => (1.0, 1.0),
                        io::IoImageFit::Contain => {
                            let scale = (composition.width as f32 / image.width() as f32)
                                .min(composition.height as f32 / image.height() as f32);
                            let width = image.width() as f32 * scale;
                            let height = image.height() as f32 * scale;
                            (
                                composition.width as f32 / width,
                                composition.height as f32 / height,
                            )
                        }
                        io::IoImageFit::Cover => {
                            let scale = (composition.width as f32 / image.width() as f32)
                                .max(composition.height as f32 / image.height() as f32);
                            let width = image.width() as f32 * scale;
                            let height = image.height() as f32 * scale;
                            (
                                composition.width as f32 / width,
                                composition.height as f32 / height,
                            )
                        }
                        io::IoImageFit::None => (
                            composition.width as f32 / image.width() as f32,
                            composition.height as f32 / image.height() as f32,
                        ),
                    };
                    let ox = (1.0 - sx) / 2.0;
                    let oy = (1.0 - sy) / 2.0;
                    let uv = three_d::VertexBuffer::new_with_data(
                        context,
                        &[
                            three_d::vec3(ox, oy, 0.0),
                            three_d::vec3(ox, oy + sy, 0.0),
                            three_d::vec3(ox + sx, oy + sy, 0.0),
                            three_d::vec3(ox, oy, 0.0),
                            three_d::vec3(ox + sx, oy + sy, 0.0),
                            three_d::vec3(ox + sx, oy, 0.0),
                        ],
                    );

                    filters.push(Filter::Image(
                        three_d::Texture2DRef::from_texture(image),
                        uv,
                    ));
                }
                io::IoFilter::Composition(composition) => {
                    let c = Box::pin(Self::load(context, composition, parent_dir)).await?;
                    filters.push(Filter::Composition(c))
                }
                _ => {
                    // Load shader filter
                    filters.push(load_shader_filter(context, filter, parent_dir));
                }
            }
        }

        let viewport = three_d::Viewport::new_at_origo(composition.width, composition.height);
        let mut camera = three_d::Camera::new_2d(viewport);

        camera.disable_tone_and_color_mapping();

        // For copy textures
        let copy_program = three_d::Program::from_source(
            context,
            "
                in vec4 a_position;
                in vec4 a_uv;
                out vec2 v_uv;
                void main() {
                  gl_Position = a_position;
                  v_uv = a_uv.xy;
                }
            ",
            "
                uniform sampler2D u_texture;
                in vec2 v_uv;
                out vec4 outColor;
                void main() {
                  if(0.0 <= v_uv.x && v_uv.x <= 1.0 && 0.0 <= v_uv.y && v_uv.y <= 1.0) {
                    outColor = texture(u_texture, v_uv);
                  } else {
                    outColor = vec4(0.0);
                  }
                }
            ",
        )
        .unwrap();

        Ok(Self {
            input: new_texture(context, composition.width, composition.height),
            output: new_texture(context, composition.width, composition.height),
            camera,
            filters,
            copy_program,
        })
    }

    fn width(&self) -> u32 {
        self.camera.viewport().width
    }

    fn height(&self) -> u32 {
        self.camera.viewport().height
    }

    /// Render the image with all applied filters
    pub fn render(&mut self, target: &mut target::Target) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;
        let clear_state = three_d::ClearState::default();
        let plane_positions = three_d::VertexBuffer::new_with_data(
            target.context(),
            &[
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(-1.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, -1.0, 0.0),
            ],
        );
        let plane_uv = three_d::VertexBuffer::new_with_data(
            target.context(),
            &[
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(0.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, 0.0, 0.0),
            ],
        );

        for filter in self.filters.iter() {
            // Apply each filter
            self.output
                .as_color_target(None)
                .clear(clear_state)
                .write(|| {
                    match filter {
                        Filter::Image(texture, uv) => {
                            self.copy_program.use_uniform_if_required(
                                "u_resolution",
                                three_d::Vector2::new(width, height),
                            );
                            self.copy_program.use_vertex_attribute("a_uv", uv);
                            self.copy_program
                                .use_vertex_attribute("a_position", &plane_positions);
                            self.copy_program.use_texture("u_texture", texture);
                            self.copy_program.draw_arrays(
                                three_d::RenderStates::default(),
                                self.camera.viewport(),
                                plane_positions.vertex_count(),
                            );
                        }
                        Filter::Shader(program, uniforms) => {
                            // Apply shader filter
                            program.use_uniform_if_required(
                                "u_resolution",
                                three_d::Vector2::new(width, height),
                            );
                            for (key, value) in uniforms {
                                program.use_uniform_if_required(key, value);
                            }
                            self.copy_program.use_vertex_attribute("a_uv", &plane_uv);
                            program.use_vertex_attribute("a_position", &plane_positions);
                            if program.requires_uniform("u_texture") {
                                program.use_texture("u_texture", &self.input);
                            }
                            program.draw_arrays(
                                three_d::RenderStates::default(),
                                self.camera.viewport(),
                                plane_positions.vertex_count(),
                            );
                        }
                        Filter::Composition(_composition) => {
                            // TODO:
                            // todo!()
                        }
                    }
                    Ok::<(), MakuError>(())
                })?;

            // Copy output to input for next filter
            self.input
                .as_color_target(None)
                .clear(clear_state)
                .write(|| {
                    self.copy_program.use_uniform_if_required(
                        "u_resolution",
                        three_d::Vector2::new(width, height),
                    );
                    self.copy_program.use_vertex_attribute("a_uv", &plane_uv);
                    self.copy_program
                        .use_vertex_attribute("a_position", &plane_positions);
                    self.copy_program.use_texture("u_texture", &self.output);
                    self.copy_program.draw_arrays(
                        three_d::RenderStates::default(),
                        self.camera.viewport(),
                        plane_positions.vertex_count(),
                    );
                    Ok::<(), MakuError>(())
                })?;
        }

        // Copy final output to the target
        target.clear(clear_state);
        target.write(|| {
            self.copy_program
                .use_uniform_if_required("u_resolution", three_d::Vector2::new(width, height));
            self.copy_program.use_vertex_attribute("a_uv", &plane_uv);
            self.copy_program
                .use_vertex_attribute("a_position", &plane_positions);
            self.copy_program.use_texture("u_texture", &self.output);
            self.copy_program.draw_arrays(
                three_d::RenderStates::default(),
                self.camera.viewport(),
                plane_positions.vertex_count(),
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

fn load_shader_filter(
    context: &three_d::Context,
    item: &io::IoFilter,
    parent_dir: &std::path::Path,
) -> Filter {
    let (vert, frag, uniforms) = match item {
        io::IoFilter::Shader { frag, vert } => (
            std::fs::read_to_string(io::resolve_resource_path(parent_dir, vert)).unwrap(),
            std::fs::read_to_string(io::resolve_resource_path(parent_dir, frag)).unwrap(),
            vec![],
        ),
        io::IoFilter::BlackWhite => (
            include_str!("./presets/blackwhite.vert").to_string(),
            include_str!("./presets/blackwhite.frag").to_string(),
            vec![],
        ),
        io::IoFilter::GaussianBlur { radius } => (
            include_str!("./presets/gaussian_blur.vert").to_string(),
            include_str!("./presets/gaussian_blur.frag").to_string(),
            vec![("u_radius".to_string(), *radius)],
        ),
        io::IoFilter::Composition(..) | io::IoFilter::Image { .. } => unreachable!(),
    };
    Filter::Shader(
        three_d::Program::from_source(context, &vert, &frag).unwrap(),
        uniforms,
    )
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