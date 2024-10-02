use clap::Parser;
use three_d::SquareMatrix;

#[derive(Parser, Debug)]
struct Args {
    /// JSON file input of filter configuration
    #[arg(long)]
    input: std::path::PathBuf,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
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

    let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

    // dummy input
    let mut loaded = three_d_asset::io::load_async(&[args.input]).await.unwrap();
    let image = three_d::Texture2D::new(&context, &loaded.deserialize("").unwrap());

    // dummy
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
            color: three_d::Srgba::WHITE,
            ..Default::default()
        },
    );
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            winit::event::Event::WindowEvent { ref event, .. } => {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
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
                    .render(&three_d::Camera::new_2d(frame_input.viewport), &model, &[]);

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
            _ => (),
        }
    });
}
