#[macro_use]
extern crate log;

mod app;
mod binds;
mod image;
mod input;
mod window;

use crate::image::{Image, ImageLoader};

fn build_logger() -> Result<(), log::SetLoggerError> {
    env_logger::builder().format_timestamp_millis().try_init()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::build_app();
    let matches = app.get_matches();
    build_logger().unwrap();

    let benchmark = matches.index_of("benchmark").is_some();
    let image_path = matches.value_of("file").unwrap().to_string();

    // Load the image from disk on separate thread
    info!("Getting image(s) from \"{}\"", image_path);
    let mut imageloader = image::ImageLoader::from_image_path(image_path)?;
    let image = std::thread::spawn(&mut || imageloader.next());

    info!("Creating window");
    let el = glutin::event_loop::EventLoop::new();
    let mut window = window::Window::new(&el)?;

    info!("Creating image object");
    let mut image = Image::from(image.join().unwrap().unwrap());

    if benchmark {
        std::process::exit(0);
    }

    info!("Creating input system");
    let input = input::Input::new(&binds::Binds::default());

    info!("Entering main loop");
    el.run(move |event, _, control_flow| {
        use glutin::{
            event::{Event, StartCause, WindowEvent},
            event_loop::ControlFlow,
        };
        match event {
            Event::NewEvents(event) => match event {
                StartCause::Init => {
                    window.request_redraw();
                    image.quad().unwrap().fit_to_window(&window);
                }
                StartCause::ResumeTimeReached {
                    start,
                    requested_resume,
                } => {
                    while image.time_next_frame().unwrap() <= requested_resume {
                        image.next_frame();
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_id) => {
                use glium::Surface;
                window
                    .draw(|frame| {
                        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                        image.draw(frame).unwrap();
                    })
                    .unwrap();
            }
            Event::MainEventsCleared => {
                if let Some(next) = image.time_next_frame() {
                    // When the event loop has processed all of the events,
                    // we want to give it a timer for our next frame
                    *control_flow = ControlFlow::WaitUntil(next);
                } else {
                    // There is no events, and therefore no more processing to do;
                    // suspend thread until new events arrive
                    *control_flow = ControlFlow::Wait;
                }
            }
            Event::LoopDestroyed => info!("Loop destroyed"),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_size) => {
                    image.quad().unwrap().fit_to_window(&window);
                }
                WindowEvent::ReceivedCharacter(c) => {
                    if input.handle_char(c, &mut image.quad().unwrap(), control_flow) {
                        window.request_redraw();
                    }
                }
                WindowEvent::KeyboardInput {
                    input: key_input, ..
                } => {
                    if input.handle(&key_input, &mut image.quad().unwrap(), control_flow) {
                        window.request_redraw();
                    }
                }
                _ => window.handle(event, control_flow),
            },
            _ => {}
        }
    });
}
