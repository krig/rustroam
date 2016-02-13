#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.position[0], self.position[1])
    }
}


fn main() {
    use glium_sdl2::DisplayBuild;
    use glium::Surface;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display = video_subsystem.window("Tutorial 01", 800, 600).resizable().build_glium().unwrap();
    let v1 = Vertex { position: [-0.5, -0.5] };
    let v2 = Vertex { position: [ 0.0,  0.5] };
    let v3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![v1, v2, v3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    fn create_shaders(display: &glium_sdl2::SDL2Facade) -> glium::Program {
        let vertex_shader_src = r#"
            #version 130

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 130

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
    }
    let program = create_shaders(&display);

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => running = false,
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vertex;

    #[test]
    fn test_vertex() {
        assert_eq!("(3, 5)", format!("{}", Vertex { position: [3.0, 5.0] }));
    }
}
