use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use glium::{program, texture, uniform};
use image::DynamicImage;
use std::path::Path;

// TODO: Add a verbosity variable
macro_rules! log_verbose {
    () => (std::eprint!("\n"));
    ($($args:tt)*) => ({
        eprintln!("[INFO]: {}", format_args!($($args)*));
    })
}

fn load_image_from_path<P>(fp: P) -> Option<DynamicImage>
where
    P: AsRef<Path>,
{
    log_verbose!("Loading image file...");
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

fn construct_texture_from_imagefile<F, P>(display: &F, fp: P) -> Option<texture::SrgbTexture2d>
where
    F: glium::backend::Facade,
    P: AsRef<Path>,
{
    if let Some(image) = load_image_from_path(fp) {
        use image::GenericImageView;
        log_verbose!("Creating texture...");

        let raw_image_data: Vec<u8> = image.to_rgba().to_vec();
        let raw_image_data = texture::RawImage2d::from_raw_rgba(raw_image_data, image.dimensions());
        let texture = texture::SrgbTexture2d::new(display, raw_image_data);
        return texture.ok();
    }
    None
}

fn main() {
    use glutin::{dpi, event_loop, window, Api, ContextBuilder, GlRequest};

    eprintln!(
        "WARNING: This application is currently considered in early-alpha and non-functional"
    );

    let start_instant = std::time::Instant::now();

    let matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("file")
                .help("Defines the file to use")
                .required(true)
                .multiple(false)
                .index(1),
        )
        .get_matches();
    let file_path = Path::new(matches.value_of("file").unwrap());

    log_verbose!(
        "Creating EventLoop: {}",
        start_instant.elapsed().as_secs_f32()
    );
    let el = event_loop::EventLoop::new();
    log_verbose!("Building Window: {}", start_instant.elapsed().as_secs_f32());
    let wb = window::WindowBuilder::new()
        .with_title(crate_name!())
        .with_inner_size(dpi::LogicalSize::new(800.0, 600.0));
    log_verbose!(
        "Building Context: {}",
        start_instant.elapsed().as_secs_f32()
    );
    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Compatibility)
        .build_windowed(wb, &el)
        .unwrap();
    log_verbose!(
        "Initializing Display: {}",
        start_instant.elapsed().as_secs_f32()
    );
    let display = glium::Display::from_gl_window(windowed_context).unwrap();

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

    log_verbose!(
        "Compiling shaders...: {}",
        start_instant.elapsed().as_secs_f32()
    );
    let program = program!(&display, 140 => {
        vertex: "
        #version 140
        in vec2 position;
        in vec2 texcoord;

        out vec4 vColor;
        out vec2 vTexCoord;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            vColor = vec4(1.0, 1.0, 1.0, 1.0);
            vTexCoord = texcoord;
        }
        ",
        fragment: "
        #version 140
        in vec4 vColor;
        in vec2 vTexCoord;

        out vec4 f_color;

        uniform sampler2D image;

        void main() {
            f_color = texture(image, vTexCoord) * vColor;
        }
        ",
    })
    .unwrap();

    let tex = construct_texture_from_imagefile(&display, &file_path).unwrap();
    log_verbose!(
        "Total time since app start: {}ms",
        start_instant.elapsed().as_millis()
    );

    el.run(move |event, _target, control| {
        use glium::Surface;
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        let draw = || {
            // build uniforms
            let sampler = tex.sampled();
            let uniforms = uniform! { texture: sampler };

            // draw a frame
            let mut frame = display.draw();
            frame.clear(None, None, false, None, None);
            frame
                .draw(&vbo, &ibo, &program, &uniforms, &Default::default())
                .unwrap();
            frame.finish().unwrap();
        };

        draw();

        match event {
            // TODO: Window resizing
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
