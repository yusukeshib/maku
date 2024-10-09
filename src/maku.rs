use clap::Parser;
use maku::error::MakuError;
use maku::Maku;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: std::path::PathBuf,
    #[arg(long)]
    output: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), MakuError> {
    let args = Args::parse();

    if let Some(output_path) = args.output {
        let context = three_d::HeadlessContext::new()?;
        let mut maku = Maku::load(&context, args.input).await?;
        maku.render(&context)?;
        maku.save_to_file(&context, output_path)?;
    } else {
        // let event_loop = winit::event_loop::EventLoop::new();
        // let window = winit::window::WindowBuilder::new()
        //     .with_max_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        //     .with_min_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        //     .with_inner_size(winit::dpi::PhysicalSize::new(args.width, args.height))
        //     .build(&event_loop)
        //     .unwrap();
        // let context = three_d::WindowedContext::from_winit_window(
        //     &window,
        //     three_d::SurfaceSettings {
        //         vsync: true,
        //         depth_buffer: 0,
        //         stencil_buffer: 0,
        //         multisamples: 4,
        //         hardware_acceleration: three_d::HardwareAcceleration::Preferred,
        //     },
        // )
        // .unwrap();

        // let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

        // event_loop.run(move |event, _, control_flow| {
        //     control_flow.set_wait();
        //     match event {
        //         winit::event::Event::WindowEvent { ref event, .. } => {
        //             frame_input_generator.handle_winit_window_event(event);
        //             match event {
        //                 winit::event::WindowEvent::Resized(physical_size) => {
        //                     context.resize(*physical_size);
        //                 }
        //                 winit::event::WindowEvent::ScaleFactorChanged {
        //                     new_inner_size, ..
        //                 } => {
        //                     context.resize(**new_inner_size);
        //                 }
        //                 winit::event::WindowEvent::CloseRequested => {
        //                     control_flow.set_exit();
        //                 }
        //                 _ => (),
        //             }
        //         }
        //         winit::event::Event::MainEventsCleared => {
        //             window.request_redraw();
        //         }
        //         winit::event::Event::RedrawRequested(_) => {
        //             let frame_input = frame_input_generator.generate(&context);
        //             frame_input
        //                 .screen()
        //                 .clear(three_d::ClearState::default())
        //                 .render(&camera, &model, &[]);
        //             context.swap_buffers().unwrap();
        //             window.request_redraw();
        //         }
        //         _ => (),
        //     }
        // });
    }
    Ok(())
}
