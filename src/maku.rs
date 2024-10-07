use clap::Parser;
use maku::error::MakuError;
use three_d::{Object, SquareMatrix};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: std::path::PathBuf,
    #[arg(long)]
    output: Option<std::path::PathBuf>,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
}

#[tokio::main]
async fn main() -> Result<(), MakuError> {
    let args = Args::parse();

    // FIXME: Not working with shaders
    // let context = three_d::HeadlessContext::new()?;

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_max_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        .with_min_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        .with_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        .build(&event_loop)
        .unwrap();
    let context = three_d::WindowedContext::from_winit_window(
        &window,
        three_d::SurfaceSettings {
            vsync: true,
            depth_buffer: 0,
            stencil_buffer: 0,
            multisamples: 4,
            hardware_acceleration: three_d::HardwareAcceleration::Preferred,
        },
    )
    .unwrap();

    let viewport = three_d::Viewport::new_at_origo(args.width, args.height);
    let camera = three_d::Camera::new_2d(viewport);

    //
    // Draw the test image to an offscreen texture
    //

    // Load a test image
    let mut loaded = three_d_asset::io::load_async(&[args.input]).await.unwrap();
    let image = three_d::Texture2D::new(&context, &loaded.deserialize("").unwrap());

    let mut tex = three_d::Texture2D::new_empty::<[u8; 4]>(
        &context,
        args.width,
        args.height,
        three_d::Interpolation::Linear,
        three_d::Interpolation::Linear,
        None,
        three_d::Wrapping::ClampToEdge,
        three_d::Wrapping::ClampToEdge,
    );
    tex.as_color_target(None).write(|| {
        let width = image.width() as f32;
        let height = image.height() as f32;

        let model = three_d::Gm::new(
            three_d::Rectangle::new(
                &context,
                three_d::vec2(width * 0.5, height * 0.5),
                three_d::degrees(0.0),
                width,
                height,
            ),
            three_d::ColorMaterial {
                texture: Some(three_d::Texture2DRef {
                    texture: image.into(),
                    transformation: three_d::Mat3::identity(),
                }),
                ..Default::default()
            },
        );

        model.render(&camera, &[]);

        Ok::<(), MakuError>(())
    })?;

    //
    // Apply filter to the offscreen texture
    //
    let mut tex2 = three_d::Texture2D::new_empty::<[u8; 4]>(
        &context,
        args.width,
        args.height,
        three_d::Interpolation::Linear,
        three_d::Interpolation::Linear,
        None,
        three_d::Wrapping::ClampToEdge,
        three_d::Wrapping::ClampToEdge,
    );
    tex2.as_color_target(None).write(|| {
        let program = three_d::Program::from_source(
            &context,
            include_str!("blackwhite.vert"),
            include_str!("blackwhite.frag"),
        )
        .unwrap();
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
                x: args.width as f32,
                y: args.height as f32,
            },
        );
        program.use_vertex_attribute("position", &positions);
        program.use_texture("u_texture", &tex);
        program.draw_arrays(
            three_d::RenderStates::default(),
            viewport,
            positions.vertex_count(),
        );
        Ok::<(), MakuError>(())
    })?;

    // Render the tex2 to screen

    let target =
        three_d::ColorTargetMultisample::<[u8; 4]>::new(&context, args.width, args.height, 4);
    target.clear(three_d::ClearState::default());
    let model = three_d::Gm::new(
        three_d::Rectangle::new(
            &context,
            three_d::vec2(args.width as f32 * 0.5, args.height as f32 * 0.5),
            three_d::degrees(0.0),
            args.width as f32,
            args.height as f32,
        ),
        three_d::ColorMaterial {
            texture: Some(tex2.into()),
            ..Default::default()
        },
    );
    target.render(&camera, &model, &[]);

    // Output

    if let Some(output_path) = args.output {
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
            args.width,
            args.height,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )?;
    } else {
        let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();
            match event {
                winit::event::Event::WindowEvent { ref event, .. } => {
                    frame_input_generator.handle_winit_window_event(event);
                    match event {
                        winit::event::WindowEvent::Resized(physical_size) => {
                            context.resize(*physical_size);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {
                            new_inner_size, ..
                        } => {
                            context.resize(**new_inner_size);
                        }
                        winit::event::WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        _ => (),
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    let frame_input = frame_input_generator.generate(&context);
                    frame_input
                        .screen()
                        .clear(three_d::ClearState::default())
                        .render(&camera, &model, &[]);
                    context.swap_buffers().unwrap();
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
    Ok(())
}
