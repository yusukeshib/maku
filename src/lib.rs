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
}

impl Maku {
    pub async fn load(
        context: &three_d::Context,
        json_path: std::path::PathBuf,
    ) -> Result<Maku, MakuError> {
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
        let camera = three_d::Camera::new_2d(viewport);

        Ok(Maku {
            input: new_texture(context, project.width, project.height),
            output: new_texture(context, project.width, project.height),
            camera,
            filters,
        })
    }

    pub fn width(&self) -> u32 {
        self.camera.viewport().width
    }

    pub fn height(&self) -> u32 {
        self.camera.viewport().height
    }

    pub fn render(&mut self, context: &three_d::Context) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        for filter in self.filters.iter() {
            self.output.as_color_target(None).write(|| {
                match filter {
                    Filter::Image(texture) => {
                        let model = three_d::Gm::new(
                            three_d::Rectangle::new(
                                context,
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
                        let positions = three_d::VertexBuffer::new_with_data(
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
                        program.use_uniform(
                            "u_resolution",
                            three_d::Vector2 {
                                x: width,
                                y: height,
                            },
                        );
                        program.use_vertex_attribute("position", &positions);
                        program.use_texture("u_texture", &self.input);
                        program.draw_arrays(
                            three_d::RenderStates::default(),
                            self.camera.viewport(),
                            positions.vertex_count(),
                        );
                    }
                }
                Ok::<(), MakuError>(())
            })?;

            // Copy output to input
            // TODO: Don't call it in render()
            {
                let copy_program = three_d::Program::from_source(
                    context,
                    include_str!("./copy.vert"),
                    include_str!("./copy.frag"),
                )
                .unwrap();
                let copy_positions = three_d::VertexBuffer::new_with_data(
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
                copy_program.use_uniform(
                    "u_resolution",
                    three_d::Vector2 {
                        x: width,
                        y: height,
                    },
                );
                copy_program.use_vertex_attribute("position", &copy_positions);
                self.input.as_color_target(None).write(|| {
                    copy_program.use_texture("u_texture", &self.output);
                    copy_program.draw_arrays(
                        three_d::RenderStates::default(),
                        self.camera.viewport(),
                        copy_positions.vertex_count(),
                    );
                    Ok::<(), MakuError>(())
                })?;
            }
        }

        Ok(())
    }

    pub fn save_to_file(
        &self,
        context: &three_d::Context,
        output_path: std::path::PathBuf,
    ) -> Result<(), MakuError> {
        let width = self.width() as f32;
        let height = self.height() as f32;

        let target = three_d::ColorTargetMultisample::<[u8; 4]>::new(
            context,
            width as u32,
            height as u32,
            4,
        );
        target.clear(three_d::ClearState::default());

        // TODO: Dedup
        target.write(|| {
            let copy_program = three_d::Program::from_source(
                context,
                include_str!("./copy.vert"),
                include_str!("./copy.frag"),
            )
            .unwrap();
            let copy_positions = three_d::VertexBuffer::new_with_data(
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
            copy_program.use_uniform(
                "u_resolution",
                three_d::Vector2 {
                    x: width,
                    y: height,
                },
            );
            copy_program.use_vertex_attribute("position", &copy_positions);
            copy_program.use_texture("u_texture", &self.output);
            copy_program.draw_arrays(
                three_d::RenderStates::default(),
                self.camera.viewport(),
                copy_positions.vertex_count(),
            );
            Ok::<(), MakuError>(())
        })?;

        let mut texture = target.resolve();
        let pixels: Vec<u8> = texture
            .as_color_target(None)
            .read::<[u8; 4]>()
            .into_iter()
            .flatten()
            .collect();
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
