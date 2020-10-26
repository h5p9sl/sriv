use crate::window::Window;
use glium::{
    backend::Facade, implement_vertex, program, texture::SrgbTexture2d, uniform, IndexBuffer,
    Program, Surface, VertexBuffer,
};
use vek::mat::repr_c::column_major::mat4::Mat4;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texcoord: [u8; 2],
}
implement_vertex!(Vertex, position, texcoord);

pub struct ImageQuad {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u8>,
    program: Program,
    matrix: Mat4<f32>,
    image_size: (f32, f32),
    texture: SrgbTexture2d,
}

impl ImageQuad {
    pub fn new<F: Facade>(display: &F, texture: SrgbTexture2d) -> ImageQuad {
        ImageQuad {
            vertex_buffer: Self::create_vbo(display),
            index_buffer: Self::create_ibo(display),
            program: Self::create_program(display),
            matrix: Mat4::<f32>::default(),
            image_size: (1.0, 1.0),
            texture,
        }
    }

    pub fn draw<S: Surface>(&self, surface: &mut S) -> Result<(), glium::DrawError> {
        let uniforms = uniform! {
            matrix: self.matrix.into_col_arrays(),
            texture: self.texture.sampled(),
            image_size: self.image_size,
        };

        surface.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default(),
        )
    }

    pub fn fit_to_window(&mut self, window: &Window) {
        let ts = self.texture.dimensions();
        let ws = window.display().get_framebuffer_dimensions();

        assert!(ts.0 >= 1 && ts.1 >= 1);
        assert!(ws.0 >= 1 && ws.1 >= 1);

        let mut s = (ts.0 as f32, ts.1 as f32);
        let ws = (ws.0 as f32, ws.1 as f32);
        if s.0 > ws.0 {
            let scaling_factor = ws.0 / s.0;
            s.0 *= scaling_factor;
            s.1 *= scaling_factor;
        }
        if s.1 > ws.1 {
            let scaling_factor = ws.1 / s.1;
            s.0 *= scaling_factor;
            s.1 *= scaling_factor;
        }

        s.0 /= ws.0;
        s.1 /= ws.1;

        self.image_size = s;
    }

    pub fn set_image<F>(&mut self, image: &::image::RgbaImage, display: &F)
    where
        F: glium::backend::Facade,
    {
        use glium::texture::{RawImage2d, SrgbTexture2d};
        let dimensions = image.dimensions();

        self.texture = {
            let raw_image = RawImage2d::from_raw_rgba(image.as_raw().to_owned(), dimensions);
            SrgbTexture2d::new(display, raw_image).unwrap()
        };
    }

    pub fn _matrix(&self) -> &Mat4<f32> {
        &self.matrix
    }

    pub fn matrix_mut(&mut self) -> &mut Mat4<f32> {
        &mut self.matrix
    }

    fn create_vbo<F: Facade>(display: &F) -> VertexBuffer<Vertex> {
        VertexBuffer::new(
            display,
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
    }

    fn create_ibo<F: Facade>(display: &F) -> IndexBuffer<u8> {
        glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TriangleStrip,
            &[0u8, 1, 2, 2, 3, 0],
        )
        .unwrap()
    }

    fn create_program<F: Facade>(display: &F) -> Program {
        program!(display, 140 => {
            vertex: "
        #version 140
        in vec2 position;
        in vec2 texcoord;
        out vec2 vTexCoord;
        uniform mat4 matrix;
        uniform vec2 image_size;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            gl_Position *= vec4(image_size, 1.0, 1.0);
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
        .unwrap()
    }
}
