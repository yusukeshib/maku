use three_d::SquareMatrix;

use crate::error::MakuError;
use crate::io;
use crate::programs;
use crate::target;
use crate::value;

/// Represents different types of filters that can be applied to an image
pub enum Filter {
    /// An composition filter
    Composition {
        composition: Composition,
        matrix: three_d::Mat3,
    },
    /// An image filter, containing a texture reference
    Image {
        texture: three_d::Texture2DRef,
        matrix: three_d::Mat3,
    },
    /// A shader filter, containing a program
    Shader {
        program: three_d::Program,
        uniforms: Vec<(String, value::UniformValue)>,
    },
}

// Composition
pub struct Composition {
    /// Input texture for processing
    input: three_d::Texture2D,
    /// Input texture for processing
    intermediate: three_d::Texture2D,
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
                    let path = io::resolve_resource_path(parent_dir, path);
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());
                    let matrix = fit_to_matrix(
                        fit,
                        image.width() as f32,
                        image.height() as f32,
                        composition.width as f32,
                        composition.height as f32,
                    );

                    filters.push(Filter::Image {
                        texture: three_d::Texture2DRef::from_texture(image),
                        matrix,
                    });
                }
                io::IoFilter::Composition(io) => {
                    let c = Box::pin(Self::load(context, io, parent_dir)).await?;
                    let matrix = fit_to_matrix(
                        &io.fit,
                        c.width as f32,
                        c.height as f32,
                        composition.width as f32,
                        composition.height as f32,
                    );

                    filters.push(Filter::Composition {
                        composition: c,
                        matrix,
                    });
                }
                _ => {
                    // Load shader filter
                    filters.push(load_shader_filter(context, filter, parent_dir));
                }
            }
        }

        Ok(Self {
            input: new_texture(context, composition.width, composition.height),
            intermediate: new_texture(context, composition.width, composition.height),
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
            programs.draw_texture(
                context,
                &self.output,
                three_d::Mat3::identity(),
                three_d::Viewport::new_at_origo(self.width, self.height),
            );
            Ok::<(), MakuError>(())
        })?;

        Ok(())
    }

    fn apply_filters(
        &mut self,
        context: &three_d::Context,
        programs: &programs::Programs,
    ) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0);
        let u_resolution = three_d::Vector2::new(self.width as f32, self.height as f32);

        for filter in self.filters.iter_mut() {
            // Apply each filter
            match filter {
                Filter::Image { texture, matrix } => {
                    self.intermediate
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            programs.draw_texture(
                                context,
                                texture,
                                *matrix,
                                three_d::Viewport::new_at_origo(self.width, self.height),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            programs.blend_textures(
                                context,
                                &self.input,
                                &self.intermediate,
                                three_d::Viewport::new_at_origo(self.width, self.height),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                }
                Filter::Shader { program, uniforms } => {
                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            let a_uv = three_d::VertexBuffer::new_with_data(
                                context,
                                &[
                                    three_d::vec2(0.0, 0.0),
                                    three_d::vec2(0.0, 1.0),
                                    three_d::vec2(1.0, 1.0),
                                    three_d::vec2(0.0, 0.0),
                                    three_d::vec2(1.0, 1.0),
                                    three_d::vec2(1.0, 0.0),
                                ],
                            );
                            let geom = three_d::VertexBuffer::new_with_data(
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

                            // Apply shader filter
                            if program.requires_uniform("u_resolution") {
                                program.use_uniform("u_resolution", u_resolution);
                            }
                            for (key, value) in uniforms.iter() {
                                if program.requires_uniform(key) {
                                    value.apply(program, key);
                                }
                            }
                            if program.requires_attribute("a_uv") {
                                program.use_vertex_attribute("a_uv", &a_uv);
                            }
                            if program.requires_attribute("a_position") {
                                program.use_vertex_attribute("a_position", &geom);
                            }
                            if program.requires_uniform("u_texture") {
                                program.use_texture("u_texture", &self.input);
                            }
                            program.draw_arrays(
                                three_d::RenderStates::default(),
                                three_d::Viewport::new_at_origo(self.width, self.height),
                                geom.vertex_count(),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                }
                Filter::Composition {
                    composition,
                    matrix,
                } => {
                    composition.apply_filters(context, programs)?;

                    self.intermediate
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            programs.draw_texture(
                                context,
                                &composition.output,
                                *matrix,
                                three_d::Viewport::new_at_origo(self.width, self.height),
                            );
                            Ok::<(), MakuError>(())
                        })?;
                    self.output
                        .as_color_target(None)
                        .clear(clear_state)
                        .write(|| {
                            programs.blend_textures(
                                context,
                                &self.input,
                                &self.intermediate,
                                three_d::Viewport::new_at_origo(self.width, self.height),
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
                    programs.draw_texture(
                        context,
                        &self.output,
                        three_d::Mat3::identity(),
                        three_d::Viewport::new_at_origo(self.width, self.height),
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
        io::IoFilter::DropShadow {
            radius,
            offset,
            color,
        } => (
            include_str!("./presets/drop_shadow.vert").to_string(),
            include_str!("./presets/drop_shadow.frag").to_string(),
            vec![
                ("u_radius".to_string(), (*radius).into()),
                ("u_offset".to_string(), (offset[0], offset[1]).into()),
                ("u_color".to_string(), (*color).into()),
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

fn fit_to_matrix(
    fit: &io::IoImageFit,
    texture_width: f32,
    texture_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) -> three_d::Mat3 {
    match fit {
        io::IoImageFit::Fill => three_d::Mat3::from_nonuniform_scale(
            viewport_width / texture_width,
            viewport_height / texture_height,
        ),
        io::IoImageFit::Contain => {
            let scale = (viewport_width / texture_width).min(viewport_height / texture_height);
            three_d::Mat3::from_scale(scale)
        }
        io::IoImageFit::Cover => {
            let scale = (viewport_width / texture_width).max(viewport_height / texture_height);
            three_d::Mat3::from_scale(scale)
        }
        io::IoImageFit::None {
            translate,
            rotate,
            scale,
        } => {
            let s = three_d::Mat3::from_nonuniform_scale(scale.x(), scale.y());
            let r = three_d::Mat3::from_angle_z(three_d::degrees(*rotate));
            let t = three_d::Mat3::from_translation(three_d::vec2(
                translate[0] / viewport_width,
                translate[1] / viewport_height,
            ));
            t * r * s
        }
    }
}
