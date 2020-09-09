use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use glium::{program, uniform};
use std::path::Path;

#[macro_use]
mod log;
mod texture;

fn main() {
    use glutin::{dpi, event_loop, window, Api, ContextBuilder, GlRequest};

    let st = std::time::Instant::now();
    macro_rules! log_verbose_t {
        ($($args:tt)*) => ({
            log_verbose!("{}: {}", st.elapsed().as_secs_f32(), format_args!($($args)*));
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
                .required(true)
                .multiple(false)
                .index(1),
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
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 1.0, 1.0);
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

    log_verbose_t!("Waiting for load_image_thread");
    let dynamic_image = load_image_thread.join().unwrap().unwrap();

    log_verbose_t!("Creating texture");
    let tex = texture::texture_from_dynamic_image(&display, &dynamic_image).unwrap();

    log_verbose_t!("Entering main loop");

    let mut matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    // Abstract this away
    let apply_scale = |matrix: &mut [[f32; 4]; 4], scale: &[f32; 4]| {
        matrix[0][0] *= scale[0];
        matrix[1][1] *= scale[1];
        matrix[2][2] *= scale[2];
        matrix[3][3] *= scale[3];
    };

    el.run(move |event, _target, control| {
        use glium::Surface;
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        let draw = || {
            // build uniforms
            let sampler = tex.sampled();
            let uniforms = uniform! { texture: sampler, matrix: matrix};

            // draw a frame
            let mut frame = display.draw();
            frame.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, Some(1.0), None);
            frame
                .draw(&vbo, &ibo, &program, &uniforms, &Default::default())
                .unwrap();
            frame.finish().unwrap();
        };

        draw();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control = ControlFlow::Exit,
                WindowEvent::Resized(size) => display.gl_window().resize(size),
                // TODO: Create functions to handle binds
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {
                        if let Some(vk) = input.virtual_keycode {
                            match vk {
                                glutin::event::VirtualKeyCode::Q => *control = ControlFlow::Exit,
                                glutin::event::VirtualKeyCode::Subtract => {
                                    apply_scale(&mut matrix, &[0.5, 0.5, 1.0, 1.0]);
                                },
                                glutin::event::VirtualKeyCode::Add => {
                                    apply_scale(&mut matrix, &[2.0, 2.0, 1.0, 1.0]);
                                },
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::LoopDestroyed => log_verbose_t!("Loop destroyed"),
            _ => {}
        };

        display.swap_buffers().unwrap();
    });
}
