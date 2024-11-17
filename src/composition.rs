use three_d::SquareMatrix;

use crate::error::MakuError;
use crate::io;
use crate::programs;
use crate::target;
use crate::value;

/// Represents different types of nodes that can be applied to an image
pub enum Node {
    /// An composition node
    Composition { composition: Composition },
    /// An image node, containing a texture reference
    Image { texture: three_d::Texture2DRef },
    /// A shader node, containing a program
    Shader {
        program: three_d::Program,
        uniforms: Vec<(String, value::UniformValue)>,
    },
}

// Composition
pub struct Composition {
    nodes: Vec<Node>,
}

impl Composition {
    pub async fn load(
        context: &three_d::Context,
        composition: &io::IoComposition,
        parent_dir: &std::path::Path,
    ) -> Result<Self, MakuError> {
        // Load resources and create nodes

        let mut nodes = vec![];
        for node in composition.nodes.iter() {
            match node {
                io::IoNode::Image(io_image) => {
                    let path = io::resolve_resource_path(parent_dir, &io_image.path);
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());
                    nodes.push(Node::Image {
                        texture: three_d::Texture2DRef::from_texture(image),
                    });
                }
                io::IoNode::Composition(io) => {
                    let composition = Box::pin(Self::load(context, io, parent_dir)).await?;
                    nodes.push(Node::Composition { composition });
                }
                _ => {
                    // Load shader node
                    nodes.push(load_shader_node(context, node, parent_dir));
                }
            }
        }

        Ok(Self { nodes })
    }

    /// Render the image with all applied nodes
    pub fn render(
        &mut self,
        context: &three_d::Context,
        target: &mut target::Target,
        programs: &programs::Programs,
    ) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::default();

        self.apply_nodes(context, programs)?;

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

    fn apply_nodes(
        &mut self,
        context: &three_d::Context,
        programs: &programs::Programs,
    ) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0);
        let u_resolution = three_d::Vector2::new(self.width as f32, self.height as f32);

        for node in self.nodes.iter_mut() {
            // Apply each node
            match node {
                Node::Image { texture, matrix } => {
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
                Node::Shader { program, uniforms } => {
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

                            // Apply shader node
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
                Node::Composition {
                    composition,
                    matrix,
                } => {
                    composition.apply_nodes(context, programs)?;

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

            // Copy output to input for next node
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

    // /// Render the image with all applied nodes and save it to a file
    // pub fn render_to_file(
    //     &mut self,
    //     context: &three_d::Context,
    //     programs: &programs::Programs,
    //     output_path: std::path::PathBuf,
    // ) -> Result<(), MakuError> {
    //     // Create a new texture for rendering
    //     let texture = new_texture(context, self.width, self.height);
    //     let mut target = target::Target::Pixels { texture };

    //     // Render to the target
    //     self.render(context, &mut target, programs)?;

    //     // Save the rendered image to a file
    //     let pixels = target.pixels();
    //     image::save_buffer_with_format(
    //         output_path,
    //         &pixels,
    //         self.width,
    //         self.height,
    //         image::ColorType::Rgba8,
    //         image::ImageFormat::Png,
    //     )?;

    //     Ok(())
    // }
}

fn load_shader_node(
    context: &three_d::Context,
    item: &io::IoNode,
    parent_dir: &std::path::Path,
) -> Node {
    let (vert, frag, uniforms) = match item {
        io::IoNode::Shader { frag, vert } => (
            std::fs::read_to_string(io::resolve_resource_path(parent_dir, vert)).unwrap(),
            std::fs::read_to_string(io::resolve_resource_path(parent_dir, frag)).unwrap(),
            vec![],
        ),
        io::IoNode::BlackWhite => (
            include_str!("./presets/blackwhite.vert").to_string(),
            include_str!("./presets/blackwhite.frag").to_string(),
            vec![],
        ),
        io::IoNode::GaussianBlur { radius } => (
            include_str!("./presets/gaussian_blur.vert").to_string(),
            include_str!("./presets/gaussian_blur.frag").to_string(),
            vec![("u_radius".to_string(), (*radius).into())],
        ),
        io::IoNode::DropShadow {
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
        io::IoNode::Composition(..) | io::IoNode::Image { .. } => unreachable!(),
    };
    Node::Shader {
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

fn transform_to_matrix(
    tr: &io::IoTransform,
    viewport_width: f32,
    viewport_height: f32,
) -> three_d::Mat3 {
    let s = three_d::Mat3::from_nonuniform_scale(tr.scale.x(), tr.scale.y());
    let r = three_d::Mat3::from_angle_z(three_d::degrees(-1.0 * tr.rotate));
    let t = three_d::Mat3::from_translation(three_d::vec2(
        tr.translate[0] / viewport_width,
        tr.translate[1] / viewport_height,
    ));
    t * r * s
}
