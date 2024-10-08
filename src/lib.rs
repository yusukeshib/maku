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
    width: u32,
    height: u32,
    filters: Vec<Filter>,
}

impl Maku {
    pub async fn load(context: &three_d::Context, json: &str) -> Result<Maku, MakuError> {
        let project = serde_json::from_str::<IoProject>(json).map_err(MakuError::from)?;

        // Load resources
        let mut filters = vec![];
        for filter in project.filters.iter() {
            match filter {
                io::IoFilter::Image { path } => {
                    let mut loaded = three_d_asset::io::load_async(&[path]).await.unwrap();
                    let image = three_d::Texture2D::new(&context, &loaded.deserialize("").unwrap());
                    filters.push(Filter::Image(three_d::Texture2DRef {
                        texture: image.into(),
                        transformation: three_d::Mat3::identity(),
                    }));
                }
                io::IoFilter::Shader { fragment, vertex } => {
                    let vert: String = vertex.into();
                    let frag: String = fragment.into();
                    let program = three_d::Program::from_source(&context, &vert, &frag).unwrap();
                    filters.push(Filter::Shader(program));
                }
            }
        }

        Ok(Maku {
            input: new_texture(context, project.width, project.height),
            output: new_texture(context, project.width, project.height),
            width: project.width,
            height: project.height,
            filters,
        })
    }

    pub fn render(&mut self, context: &three_d::Context) -> Result<(), MakuError> {
        let viewport = three_d::Viewport::new_at_origo(self.width, self.height);
        let camera = three_d::Camera::new_2d(viewport);

        for filter in self.filters.iter() {
            match filter {
                Filter::Image(texture) => {
                    self.output.as_color_target(None).write(|| {
                        let width = self.width as f32;
                        let height = self.height as f32;

                        let model = three_d::Gm::new(
                            three_d::Rectangle::new(
                                &context,
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

                        model.render(&camera, &[]);

                        Ok::<(), MakuError>(())
                    })?;
                }
                Filter::Shader(program) => {
                    let positions = three_d::VertexBuffer::new_with_data(
                        &context,
                        &[
                            three_d::vec3(-1.0, -1.0, 0.0),
                            three_d::vec3(-1.0, 1.0, 0.0),
                            three_d::vec3(1.0, 1.0, 0.0),
                            three_d::vec3(-1.0, -1.0, 0.0),
                            three_d::vec3(1.0, 1.0, 0.0),
                            three_d::vec3(1.0, -1.0, 0.0),
                        ],
                    );
                    program.use_uniform(
                        "u_resolution",
                        three_d::Vector2 {
                            x: self.width as f32,
                            y: self.height as f32,
                        },
                    );
                    program.use_vertex_attribute("position", &positions);
                    program.use_texture("u_texture", &self.input);
                    program.draw_arrays(
                        three_d::RenderStates::default(),
                        viewport,
                        positions.vertex_count(),
                    );
                }
            }

            // TODO: Copy(Blit) output to input
        }

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
