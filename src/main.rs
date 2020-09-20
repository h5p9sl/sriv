#[macro_use]
extern crate log;

mod app;
mod binds;
mod imagequad;
mod input;
mod texture;
mod window;

fn build_logger() -> Result<(), log::SetLoggerError> {
    env_logger::builder().format_timestamp_millis().try_init()
}

fn main() -> Result<(), String> {
    let app = app::build_app();
    let matches = app.get_matches();
    build_logger().unwrap();

    let benchmark = matches.index_of("benchmark").is_some();
    let tex_path = matches.value_of("file").unwrap().to_string();
    info!("Loading image");
    let image = std::thread::spawn(move || texture::Image::new(tex_path));

    info!("Creating window");
    let el = glutin::event_loop::EventLoop::new();
    let mut window = window::Window::new(&el)?;

    info!("Creating texture");
    let image = image.join().unwrap().unwrap();
    let texture =
        texture::texture_from_dynamic_image(window.display(), &image.image.unwrap()).unwrap();
    info!("Creating imagequad");
    let mut image_quad = imagequad::ImageQuad::new(window.display(), texture);

    if benchmark {
        std::process::exit(0);
    }

    info!("Creating input system");
    let binds = binds::Binds::default();
    let input = input::Input::new(&binds);

    info!("Entering main loop");
    el.run(move |event, _, control_flow| {
        use glutin::event::{Event, StartCause, WindowEvent};
        match event {
            Event::NewEvents(event) => match event {
                StartCause::Init => {
                    window.request_redraw();
                    image_quad.fit_to_window(&window);
                }
                _ => {}
            },
            Event::RedrawRequested(_id) => {
                use glium::Surface;
                window
                    .draw(|frame| {
                        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                        image_quad.draw(frame).unwrap();
                    })
                    .unwrap();
            }
            Event::LoopDestroyed => info!("Loop destroyed"),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_size) => {
                    image_quad.fit_to_window(&window);
                }
                WindowEvent::ReceivedCharacter(c) => {
                    if input.handle_char(c, &mut image_quad, control_flow) {
                        window.request_redraw();
                    }
                }
                WindowEvent::KeyboardInput {
                    input: key_input, ..
                } => {
                    if input.handle(&key_input, &mut image_quad, control_flow) {
                        window.request_redraw();
                    }
                }
                _ => window.handle(event, control_flow),
            },
            _ => {}
        }
    });
}
