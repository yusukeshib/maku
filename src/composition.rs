use crate::error::MakuError;
use crate::io;
use crate::target;

/// Represents different types of filters that can be applied to an image
pub enum Filter {
    /// An composition filter
    Composition {
        composition: Composition,
        uv: three_d::VertexBuffer,
    },
    /// An image filter, containing a texture reference
    Image {
        texture: three_d::Texture2DRef,
        uv: three_d::VertexBuffer,
    },
    /// A shader filter, containing a program
    Shader {
        program: three_d::Program,
        uniforms: Vec<(String, f32)>,
    },
}

// Composition
pub struct Composition {
    /// Input texture for processing
    input: three_d::Texture2D,
    /// Output texture after processing
    output: three_d::Texture2D,
    /// Width of the composition
    width: u32,
    /// Width of the composition
    height: u32,
    /// List of filters to be applied
    filters: Vec<Filter>,

    // TODO: These should be global
    /// Program for copying textures
    copy_program: three_d::Program,
    /// Program for blend textures
    blend_program: three_d::Program,
}

// TODO: Dedup many lines

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

                    let uv = new_uv(
                        context,
                        composition.width as f32,
                        composition.height as f32,
                        image.width() as f32,
                        image.height() as f32,
                        fit,
                    );

                    filters.push(Filter::Image {
                        texture: three_d::Texture2DRef::from_texture(image),
                        uv,
                    });
                }
                io::IoFilter::Composition(io) => {
                    let c = Box::pin(Self::load(context, io, parent_dir)).await?;

                    let uv = new_uv(
                        context,
                        composition.width as f32,
                        composition.height as f32,
                        c.width as f32,
                        c.height as f32,
                        &io.fit,
                    );

                    filters.push(Filter::Composition { composition: c, uv });
                }
                _ => {
                    // Load shader filter
                    filters.push(load_shader_filter(context, filter, parent_dir));
                }
            }
        }

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

        // For blend textures
        let blend_program = three_d::Program::from_source(
            context,
            "
                in vec4 a_position;
                in vec4 a_uv1;
                in vec4 a_uv2;
                out vec2 v_uv1;
                out vec2 v_uv2;
                void main() {
                  gl_Position = a_position;
                  v_uv1 = a_uv1.xy;
                  v_uv2 = a_uv2.xy;
                }
            ",
            "
                uniform sampler2D u_texture1;
                uniform sampler2D u_texture2;
                in vec2 v_uv1;
                in vec2 v_uv2;
                out vec4 outColor;
                void main() {
                  if(0.0 <= v_uv2.x && v_uv2.x <= 1.0 && 0.0 <= v_uv2.y && v_uv2.y <= 1.0) {
                    vec4 c1 = texture(u_texture1, v_uv1);
                    vec4 c2 = texture(u_texture2, v_uv2);
                    outColor = c2 * c2.a + c1 * (1.0-c2.a);
                  } else {
                    outColor = texture(u_texture1, v_uv1);
                  }
                }
            ",
        )
        .unwrap();

        Ok(Self {
            input: new_texture(context, composition.width, composition.height),
            output: new_texture(context, composition.width, composition.height),
            width: composition.width,
            height: composition.height,
            filters,
            copy_program,
            blend_program,
        })
    }

    /// Render the image with all applied filters
    pub fn render(&mut self, target: &mut target::Target) -> Result<(), MakuError> {
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

        self.apply_filters(target.context())?;

        // Copy final output to the target
        target.clear(clear_state);
        target.write(|| {
            if self.copy_program.requires_attribute("a_uv") {
                self.copy_program.use_vertex_attribute("a_uv", &plane_uv);
            }
            if self.copy_program.requires_attribute("a_position") {
                self.copy_program
                    .use_vertex_attribute("a_position", &plane_positions);
            }
            if self.copy_program.requires_uniform("u_texture") {
                self.copy_program.use_texture("u_texture", &self.output);
            }
            self.copy_program.draw_arrays(
                three_d::RenderStates::default(),
                three_d::Viewport::new_at_origo(self.width, self.height),
                plane_positions.vertex_count(),
            );
            Ok::<(), MakuError>(())
        })?;

        Ok(())
    }

    fn apply_filters(&mut self, context: &three_d::Context) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::default();
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
        let plane_uv = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(0.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, 0.0, 0.0),
            ],
        );

        let u_resolution = three_d::Vector2::new(self.width as f32, self.height as f32);

        for filter in self.filters.iter_mut() {
            // Apply each filter
            match filter {
                Filter::Image { texture, uv } => {
                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            if self.blend_program.requires_attribute("a_uv1") {
                                self.blend_program.use_vertex_attribute("a_uv1", &plane_uv);
                            }
                            if self.blend_program.requires_attribute("a_uv2") {
                                self.blend_program.use_vertex_attribute("a_uv2", uv);
                            }
                            if self.blend_program.requires_attribute("a_position") {
                                self.blend_program
                                    .use_vertex_attribute("a_position", &plane_positions);
                            }
                            if self.blend_program.requires_uniform("u_texture1") {
                                self.blend_program.use_texture("u_texture1", &self.input);
                            }
                            if self.blend_program.requires_uniform("u_texture2") {
                                self.blend_program.use_texture("u_texture2", texture);
                            }
                            self.blend_program.draw_arrays(
                                three_d::RenderStates::default(),
                                three_d::Viewport::new_at_origo(self.width, self.height),
                                plane_positions.vertex_count(),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                }
                Filter::Shader { program, uniforms } => {
                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            // Apply shader filter
                            if program.requires_uniform("u_resolution") {
                                program.use_uniform("u_resolution", u_resolution);
                            }
                            for (key, value) in uniforms.iter() {
                                if program.requires_uniform(key) {
                                    program.use_uniform(key, value);
                                }
                            }
                            if program.requires_attribute("a_uv") {
                                program.use_vertex_attribute("a_uv", &plane_uv);
                            }
                            if program.requires_attribute("a_position") {
                                program.use_vertex_attribute("a_position", &plane_positions);
                            }
                            if program.requires_uniform("u_texture") {
                                program.use_texture("u_texture", &self.input);
                            }
                            program.draw_arrays(
                                three_d::RenderStates::default(),
                                three_d::Viewport::new_at_origo(self.width, self.height),
                                plane_positions.vertex_count(),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                }
                Filter::Composition { composition, uv } => {
                    composition.apply_filters(context)?;

                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            if self.blend_program.requires_attribute("a_uv1") {
                                self.blend_program.use_vertex_attribute("a_uv1", &plane_uv);
                            }
                            if self.blend_program.requires_attribute("a_uv2") {
                                self.blend_program.use_vertex_attribute("a_uv2", uv);
                            }
                            if self.blend_program.requires_attribute("a_position") {
                                self.blend_program
                                    .use_vertex_attribute("a_position", &plane_positions);
                            }
                            if self.blend_program.requires_uniform("u_texture1") {
                                self.blend_program.use_texture("u_texture1", &self.input);
                            }
                            if self.blend_program.requires_uniform("u_texture2") {
                                self.blend_program
                                    .use_texture("u_texture2", &composition.output);
                            }
                            self.blend_program.draw_arrays(
                                three_d::RenderStates::default(),
                                three_d::Viewport::new_at_origo(self.width, self.height),
                                plane_positions.vertex_count(),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                }
            }

            // Copy output to input for next filter
            self.input
                .as_color_target(None)
                .clear(clear_state)
                .write(|| {
                    if self.copy_program.requires_attribute("a_uv") {
                        self.copy_program.use_vertex_attribute("a_uv", &plane_uv);
                    }
                    if self.copy_program.requires_attribute("a_position") {
                        self.copy_program
                            .use_vertex_attribute("a_position", &plane_positions);
                    }
                    if self.copy_program.requires_uniform("u_texture") {
                        self.copy_program.use_texture("u_texture", &self.output);
                    }
                    self.copy_program.draw_arrays(
                        three_d::RenderStates::default(),
                        three_d::Viewport::new_at_origo(self.width, self.height),
                        plane_positions.vertex_count(),
                    );
                    Ok::<(), MakuError>(())
                })?;
        }

        Ok(())
    }

    /// Render the image with all applied filters and save it to a file
    pub fn render_to_file(
        &mut self,
        context: &three_d::Context,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        // Create a new texture for rendering
        let texture = new_texture(context, self.width, self.height);
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
            self.width,
            self.height,
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
    Filter::Shader {
        program: three_d::Program::from_source(context, &vert, &frag).unwrap(),
        uniforms,
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

fn new_uv(
    context: &three_d::Context,
    composition_width: f32,
    composition_height: f32,
    image_width: f32,
    image_height: f32,
    fit: &io::IoImageFit,
) -> three_d::VertexBuffer {
    let (sx, sy) = match fit {
        io::IoImageFit::Fill => (1.0, 1.0),
        io::IoImageFit::Contain => {
            let scale = (composition_width / image_width).min(composition_height / image_height);
            let width = image_width * scale;
            let height = image_height * scale;
            (composition_width / width, composition_height / height)
        }
        io::IoImageFit::Cover => {
            let scale = (composition_width / image_width).max(composition_height / image_height);
            let width = image_width * scale;
            let height = image_height * scale;
            (composition_width / width, composition_height / height)
        }
        io::IoImageFit::None => (
            composition_width / image_width,
            composition_height / image_height,
        ),
    };
    let ox = (1.0 - sx) / 2.0;
    let oy = (1.0 - sy) / 2.0;
    three_d::VertexBuffer::new_with_data(
        context,
        &[
            three_d::vec3(ox, oy, 0.0),
            three_d::vec3(ox, oy + sy, 0.0),
            three_d::vec3(ox + sx, oy + sy, 0.0),
            three_d::vec3(ox, oy, 0.0),
            three_d::vec3(ox + sx, oy + sy, 0.0),
            three_d::vec3(ox + sx, oy, 0.0),
        ],
    )
}
