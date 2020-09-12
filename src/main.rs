use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use glium::{program, uniform};
use std::path::Path;
use vek::mat::repr_c::column_major::mat4;

#[macro_use]
mod log;
mod binds;
mod texture;

fn main() {
    use glutin::{dpi, event_loop, window, Api, ContextBuilder, GlRequest};

    let st = std::time::Instant::now();
    macro_rules! log_verbose_t {
        ($($args:tt)*) => ({
            log_verbose!("{}: {}", st.elapsed().as_secs_f32().to_string().green(), format_args!($($args)*));
        })
    }

    use colored::*;
    eprintln!(
        "{}: This application is currently considered in early-alpha and non-functional",
        "WARNING".red()
    );

    let matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("file")
                .help("Defines the file to use")
                .multiple(false)
                .required(true)
        )
        .arg(
            Arg::with_name("benchmark")
                .help("Exits upon loading/initializing everything")
                .long_help(
                    "Exits upon loading and initializing everything needed to start displaying the image on the screen. Use this option to get how long it took to load the image, initialize OpenGL, etc.",
                )
                .long("benchmark")
                .short("B")
                .required(false),
        )
        .get_matches();

    let file_path = Path::new(matches.value_of("file").unwrap()).to_owned();

    log_verbose_t!("Loading image file(s)");
    let load_image_thread = std::thread::spawn(move || texture::dynamic_image_from_path(file_path));

    log_verbose_t!("Creating EventLoop");
    let el = event_loop::EventLoop::new();

    log_verbose_t!("Building Window");
    let wb = window::WindowBuilder::new()
        .with_title(crate_name!())
        .with_inner_size(dpi::LogicalSize::new(800.0, 600.0));

    log_verbose_t!("Building Context");
    let cb = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Compatibility)
        .with_srgb(true);

    log_verbose_t!("Initializing Display");
    let display = glium::Display::new(wb, cb, &el).unwrap();

    let vbo = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            texcoord: [i32; 2],
        }
        glium::implement_vertex!(Vertex, position, texcoord);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    texcoord: [0, 1],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    texcoord: [0, 0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    texcoord: [1, 0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    texcoord: [1, 1],
                },
            ],
        )
        .unwrap()
    };

    let ibo = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleStrip,
        &[0u16, 1, 2, 2, 3, 0],
    )
    .unwrap();

    log_verbose_t!("Compiling shaders");
    let program = program!(&display, 140 => {
        vertex: "
        #version 140
        in vec2 position;
        in vec2 texcoord;
        out vec2 vTexCoord;
        uniform mat4 model;
        void main() {
            gl_Position = model * vec4(position, 0.0, 1.0);
            vTexCoord = texcoord;
        }
        ",
        fragment: "
        #version 140
        in vec2 vTexCoord;
        out vec4 f_color;
        uniform sampler2D image;
        void main() {
            f_color = texture(image, vTexCoord);
        }
        ",
    })
    .unwrap();

    let tex = {
        log_verbose_t!("Waiting for load_image_thread");
        let dynamic_image = load_image_thread.join().unwrap().unwrap();

        log_verbose_t!("Creating texture");
        texture::texture_from_dynamic_image(&display, &dynamic_image).unwrap()
    };

    log_verbose_t!("Entering main loop");
    matches.index_of("benchmark").and_then(|_| -> Option<()> {
        eprintln!(
            "{}: Benchmark mode is enabled - the program will exit immediately",
            "WARNING".red()
        );
        println!("{}", st.elapsed().as_secs_f32());
        std::process::exit(0);
    });

    let matrix = mat4::Mat4::<f32>::default();
    let mut model = matrix.clone();

    let keybinds = binds::Binds::default();

    el.run(move |event, _target, control| {
        use glium::Surface;
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        use std::ops::Deref;

        let gl_window = display.gl_window();
        let windowed_context = gl_window.deref().deref();

        let draw = || {
            // build uniforms
            let sampler = tex.sampled();
            let uniforms = uniform! {
                texture: sampler,
                model: model.into_col_arrays(),
            };

            // draw a frame
            let mut frame = display.draw();
            frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            frame
                .draw(&vbo, &ibo, &program, &uniforms, &Default::default())
                .unwrap();
            frame.finish().unwrap();
        };

        match event {
            Event::RedrawRequested(_id) => draw(),
            Event::LoopDestroyed => log_verbose_t!("Loop destroyed"),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control = ControlFlow::Exit,
                WindowEvent::Resized(size) => windowed_context.resize(size),
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {
                        if let Some(vk) = input.virtual_keycode {
                            use binds::Action;
                            use std::f32::consts::PI;

                            windowed_context.window().request_redraw();
                            if let Some(action) = keybinds.get_action(vk) {
                                match action {
                                    Action::Quit => *control = ControlFlow::Exit,
                                    Action::Reset => model = matrix,
                                    Action::MoveDown => model.translate_2d([0.0, 1.0]),
                                    Action::MoveUp => model.translate_2d([0.0, -1.0]),
                                    Action::MoveLeft => model.translate_2d([1.0, 0.0]),
                                    Action::MoveRight => model.translate_2d([-1.0, 0.0]),
                                    Action::ZoomIn => model.scale_3d([2.0, 2.0, 1.0]),
                                    Action::ZoomOut => model.scale_3d([0.5, 0.5, 1.0]),
                                    Action::RotateRight => model.rotate_z(PI / 2.0),
                                    Action::RotateLeft => model.rotate_z(-PI / 2.0),
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };

        display.swap_buffers().unwrap();
    });
}
