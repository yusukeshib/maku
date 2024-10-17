use crate::error::MakuError;
use crate::io;
use crate::programs;
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
        uniforms: Vec<(String, UniformValue)>,
    },
}

enum UniformValue {
    Float(f32),
    Vector2(three_d::Vector2<f32>),
    Vector3(three_d::Vector3<f32>),
    Vector4(three_d::Vector4<f32>),
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<(f32, f32)> for UniformValue {
    fn from(value: (f32, f32)) -> Self {
        Self::Vector2(three_d::Vector2::new(value.0, value.1))
    }
}

impl From<(f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::Vector3(three_d::Vector3::new(value.0, value.1, value.2))
    }
}

impl From<(f32, f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::Vector4(three_d::Vector4::new(value.0, value.1, value.2, value.3))
    }
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

        Ok(Self {
            input: new_texture(context, composition.width, composition.height),
            output: new_texture(context, composition.width, composition.height),
            width: composition.width,
            height: composition.height,
            filters,
        })
    }

    /// Render the image with all applied filters
    pub fn render(
        &mut self,
        context: &three_d::Context,
        target: &mut target::Target,
        programs: &programs::Programs,
    ) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::default();

        self.apply_filters(context, programs)?;

        // Copy final output to the target
        target.clear(context, clear_state);
        target.write(context, || {
            programs.copy(context, &self.output);
            Ok::<(), MakuError>(())
        })?;

        Ok(())
    }

    fn apply_filters(
        &mut self,
        context: &three_d::Context,
        programs: &programs::Programs,
    ) -> Result<(), MakuError> {
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
                            programs.blend(context, &self.input, texture, uv);
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
                    composition.apply_filters(context, programs)?;

                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            programs.blend(context, &self.input, &composition.output, uv);
                            Ok::<(), MakuError>(())
                        })?;
                }
            }

            // Copy output to input for next filter
            self.input
                .as_color_target(None)
                .clear(clear_state)
                .write(|| {
                    programs.copy(context, &self.output);
                    Ok::<(), MakuError>(())
                })?;
        }

        Ok(())
    }

    /// Render the image with all applied filters and save it to a file
    pub fn render_to_file(
        &mut self,
        context: &three_d::Context,
        programs: &programs::Programs,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        // Create a new texture for rendering
        let texture = new_texture(context, self.width, self.height);
        let mut target = target::Target::Pixels { texture };

        // Render to the target
        self.render(context, &mut target, programs)?;

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
            vec![("u_radius".to_string(), (*radius).into())],
        ),
        io::IoFilter::DropShadow { radius, x, y } => (
            include_str!("./presets/drop_shadow.vert").to_string(),
            include_str!("./presets/drop_shadow.frag").to_string(),
            vec![
                ("u_radius".to_string(), (*radius).into()),
                ("u_offset".to_string(), (*x, *y).into()),
            ],
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
