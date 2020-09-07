use clap::Arg;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use glium::{program, uniform};
use image::DynamicImage;
use std::path::Path;

fn _load_image_from_path<P>(fp: P) -> Option<DynamicImage>
where
    P: AsRef<Path>,
{
    let fp = fp.as_ref();
    let di = image::open(&fp);
    if let Some(e) = di.as_ref().err() {
        eprintln!(
            "Failed to open image \"{}\": {}...",
            fp.to_str().unwrap_or("null"),
            e
        );
    }
    di.ok()
}

fn _construct_texture_from_imagefile<P>(_fp: P) -> Option<glium::Texture2d>
where
    P: AsRef<Path>,
{
    todo!();
}

fn main() {
    use glutin::{dpi, event_loop, window, Api, ContextBuilder, GlRequest};

    println!("WARNING: This application is currently considered in early-alpha and non-functional");

    let _matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("file")
                .help("Defines the file to use")
                .required(false)
                .multiple(false)
                .index(1),
        )
        .get_matches();

    let el = event_loop::EventLoop::new();
    let wb = window::WindowBuilder::new()
        .with_title(crate_name!())
        .with_inner_size(dpi::LogicalSize::new(800.0, 600.0));
    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Compatibility)
        .build_windowed(wb, &el)
        .unwrap();
    let display = glium::Display::from_gl_window(windowed_context).unwrap();

    let vbo = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        glium::implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                },
                Vertex {
                    position: [-0.5, 0.5],
                },
                Vertex {
                    position: [0.5, 0.5],
                },
                Vertex {
                    position: [0.5, -0.5],
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

    let program = program!(&display, 140 => {
        vertex: "
        #version 140
        in vec2 position;

        out vec3 vColor;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            vColor = vec3(1.0, 1.0, 1.0);
        }
        ",
        fragment: "
        #version 140
        in vec3 vColor;
        out vec4 f_color;

        void main() {
            f_color = vec4(vColor, 1.0);
        }
        ",
    })
    .unwrap();

    let uniforms = uniform! {};

    el.run(move |event, _target, control| {
        use glium::Surface;
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        let mut frame = display.draw();
        frame.clear(None, None, false, None, None);
        frame
            .draw(&vbo, &ibo, &program, &uniforms, &Default::default())
            .unwrap();
        frame.finish().unwrap();

        match event {
            Event::WindowEvent { event: we, .. } => {
                if let WindowEvent::CloseRequested = we {
                    *control = ControlFlow::Exit;
                }
            }
            Event::LoopDestroyed => println!("Loop destroyed"),
            _ => {}
        }

        display.swap_buffers().unwrap();
    });
}
