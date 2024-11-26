pub mod error;
pub mod io;
pub mod programs;
pub mod target;
pub mod value;

use error::MakuError;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Property<T> {
    Value(T),
    // TODO: Not okay
    Link(Rc<RefCell<T>>),
}

// TODO: These functions are ugly..
impl<T> Property<T> {
    fn value<K>(&self, callback: impl FnOnce(&T) -> K) -> K {
        match self {
            Property::Value(value) => callback(value),
            Property::Link(link) => {
                let value = link.borrow();
                callback(&value)
            }
        }
    }
    fn value_mut<K>(&mut self, callback: impl FnOnce(&mut T) -> K) -> K {
        match self {
            Property::Value(value) => callback(value),
            Property::Link(link) => {
                let mut value = link.borrow_mut();
                callback(&mut value)
            }
        }
    }
}

pub enum Node {
    Image {
        width: Property<f32>,
        height: Property<f32>,
        output: Property<three_d::Texture2D>,
    },
    BlackWhite {
        program: three_d::Program,
        input: Property<three_d::Texture2D>,
        output: Property<three_d::Texture2D>,
    },
    GaussianBlur {
        radius: Property<f32>,
        program: three_d::Program,
        input: Property<three_d::Texture2D>,
        output: Property<three_d::Texture2D>,
    },
    // DropShadow {
    //     radius: f32,
    //     offset: [f32; 2],
    //     color: [f32; 4],
    // },
    // Shader {
    //     program: three_d::Program,
    //     uniforms: Vec<(String, value::UniformValue)>,
    // },
    // Composition { composition: Composition },
}

/// Main structure for the Maku image processing system
pub struct Maku {
    nodes: Vec<Node>,
    programs: programs::Programs,
}

impl Maku {
    pub async fn load(
        context: &three_d::Context,
        composition: &io::IoComposition,
        parent_dir: &std::path::Path,
    ) -> Result<Self, MakuError> {
        // TODO: Load link information first
        // TODO: Disable orphaned nodes

        let mut nodes = vec![];
        for node in composition.nodes.iter() {
            match &node.node {
                io::IoNode::Image { path } => {
                    let path = io::resolve_resource_path(parent_dir, path);
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(context, &loaded.deserialize("").unwrap());
                    nodes.push(Node::Image {
                        width: Property::Value(image.width() as f32),
                        height: Property::Value(image.height() as f32),
                        output: Property::Value(image),
                    });
                }
                io::IoNode::BlackWhite => {
                    let vert = include_str!("./presets/blackwhite.vert").to_string();
                    let frag = include_str!("./presets/blackwhite.frag").to_string();

                    // TODO: Check link information
                    let input = new_texture(context, 640, 640);
                    let output = new_texture(context, input.width(), input.height());

                    nodes.push(Node::BlackWhite {
                        program: three_d::Program::from_source(context, &vert, &frag).unwrap(),
                        input: Property::Value(input),
                        output: Property::Value(output),
                    });
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            nodes,
            programs: programs::Programs::new(context),
        })
    }

    /// Render the image with all applied nodes
    pub fn render(&mut self, context: &three_d::Context) -> Result<(), MakuError> {
        self.apply_nodes(context)?;

        // // Copy final output to the target
        // let clear_state = three_d::ClearState::default();
        // target.clear(context, clear_state);
        // target.write(context, || {
        //     programs.draw_texture(
        //         context,
        //         &self.output,
        //         three_d::Mat3::identity(),
        //         three_d::Viewport::new_at_origo(self.width, self.height),
        //     );
        //     Ok::<(), MakuError>(())
        // })?;

        Ok(())
    }

    fn apply_nodes(&mut self, context: &three_d::Context) -> Result<(), MakuError> {
        let clear_state = three_d::ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0);

        for node in self.nodes.iter_mut() {
            // Apply each node
            match node {
                Node::BlackWhite {
                    program,
                    input,
                    output,
                } => {
                    let width = output.value(|v| v.width());
                    let height = output.value(|v| v.height());
                    output.value_mut(|v| {
                        v.as_color_target(None).clear(clear_state).write(|| {
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
                            program.use_uniform(
                                "u_resolution",
                                three_d::Vector2::new(width as f32, height as f32),
                            );
                            program.use_vertex_attribute("a_uv", &a_uv);
                            program.use_vertex_attribute("a_position", &geom);
                            input.value(|v| program.use_texture("u_texture", v));
                            program.draw_arrays(
                                three_d::RenderStates::default(),
                                three_d::Viewport::new_at_origo(width, height),
                                geom.vertex_count(),
                            );
                            Ok::<(), MakuError>(())
                        })
                    })?;
                }
                Node::GaussianBlur { .. } => {
                    // TODO:
                    todo!()
                }
                Node::Image { .. } => {
                    // Do nothing
                }
            }
        }

        Ok(())
    }
}

fn new_texture(context: &three_d::Context, width: u32, height: u32) -> three_d::Texture2D {
    three_d::Texture2D::new_empty::<u8>(
        context,
        width,
        height,
        three_d::Interpolation::Nearest,
        three_d::Interpolation::Nearest,
        None,
        three_d::Wrapping::ClampToEdge,
        three_d::Wrapping::ClampToEdge,
    )
}
