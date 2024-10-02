use clap::Parser;

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

fn main() {
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

    let mut camera = three_d::Camera::new_perspective(
        three_d::Viewport::new_at_origo(1, 1),
        three_d::vec3(0.0, 2.0, 4.0),
        three_d::vec3(0.0, 0.0, 0.0),
        three_d::vec3(0.0, 1.0, 0.0),
        three_d::degrees(45.0),
        0.1,
        10.0,
    );
    let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

    // dummy
    let model = three_d::Gm::new(
        three_d::Mesh::new(&context, &three_d::CpuMesh::cube()),
        three_d::ColorMaterial {
            color: three_d::Srgba::GREEN,
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

                camera.set_viewport(frame_input.viewport);
                // model.animate(frame_input.accumulated_time as f32);
                frame_input
                    .screen()
                    .clear(three_d::ClearState::color_and_depth(
                        0.8, 0.8, 0.8, 1.0, 1.0,
                    ))
                    .render(&camera, &model, &[]);

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
            _ => (),
        }
    });
}
