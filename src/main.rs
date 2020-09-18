#[macro_use]
mod log;

mod app;
mod binds;
mod imagequad;
mod input;
mod texture;
mod window;

fn main() -> Result<(), String> {
    let st = std::time::Instant::now();
    macro_rules! log_verbose_t {
        ($($args:tt)*) => ({
            log_verbose!("{}: {}", st.elapsed().as_secs_f32().to_string().green(), format_args!($($args)*));
        })
    }

    let app = app::build_app();
    let matches = app.get_matches();

    let benchmark = matches.index_of("benchmark").is_some();
    let tex_path = matches.value_of("file").unwrap().to_string();
    let texture = std::thread::spawn(move || texture::dynamic_image_from_path(tex_path));

    log_verbose_t!("Creating window");
    let el = glutin::event_loop::EventLoop::new();
    let mut window = window::Window::new(&el)?;

    log_verbose_t!("Creating texture");
    let texture =
        texture::texture_from_dynamic_image(window.display(), &texture.join().unwrap().unwrap())
            .unwrap();
    let mut image_quad = imagequad::ImageQuad::new(window.display(), texture);

    if benchmark {
        std::process::exit(0);
    }

    log_verbose_t!("Creating input system");
    let binds = binds::Binds::default();
    let input = input::Input::new(&binds);

    log_verbose_t!("Entering main loop");
    el.run(move |event, _, control_flow| {
        use glutin::event::{Event, StartCause, WindowEvent};
        match event {
            Event::NewEvents(event) => match event {
                StartCause::Init => window.request_redraw(),
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
            Event::LoopDestroyed => log_verbose!("Loop destroyed"),
            Event::WindowEvent { event, .. } => match event {
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
