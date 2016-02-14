#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

use std::{fmt, io};
use glium::{Surface, VertexBuffer, Program, index, uniforms};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use glium_sdl2::{DisplayBuild, SDL2Facade};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.position[0], self.position[1])
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    ProgramCreation(glium::program::ProgramCreationError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::ProgramCreation(ref err) => err.fmt(f)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::ProgramCreation(ref err) => err.description()
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<glium::program::ProgramCreationError> for Error {
    fn from(err: glium::program::ProgramCreationError) -> Error {
        Error::ProgramCreation(err)
    }
}

type Result<T> = std::result::Result<T, Error>;


fn handle_key(keycode: Keycode) -> bool {
    match keycode {
        Keycode::Escape => false,
        _ => true,
    }
}

fn load_file(name: &str) -> Result<String> {
    use std::io::prelude::*;
    use std::fs::File;
    let (mut f, mut s) = (try!(File::open(name)), String::new());
    try!(f.read_to_string(&mut s));
    Ok(s)
}


fn main() {
    let sdl_context = sdl2::init()
        .unwrap();
    let video_subsystem = sdl_context
        . video()
        .unwrap();
    let display = video_subsystem
        .window("rustroam", 1024, 576)
        .resizable()
        .build_glium()
        .unwrap();

    let v1 = Vertex { position: [-0.5, -0.5] };
    let v2 = Vertex { position: [ 0.0,  0.5] };
    let v3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![v1, v2, v3];

    let vertex_buffer = VertexBuffer::new(&display, &shape)
        .unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    fn create_shaders(display: &SDL2Facade) -> Result<Program> {
        let vertex_shader_src = try!(load_file("shaders/basic.vert"));
        let fragment_shader_src = try!(load_file("shaders/basic.frag"));
        let program = try!(Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None));
        Ok(program)
    }
    let program = create_shaders(&display).unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let mut target = display.draw();
        target.clear_color(0.05, 0.05, 0.05, 1.0);
        target.draw(&vertex_buffer,
                    &indices,
                    &program,
                    &uniforms::EmptyUniforms,
                    &Default::default())
            .unwrap();
        target
            .finish()
            .unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => running = handle_key(keycode.unwrap()),
                Event::Quit { .. } => running = false,
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Vertex, handle_key};
    use sdl2::keyboard::Keycode;

    #[test]
    fn test_vertex() {
        assert_eq!("(3, 5)", format!("{}", Vertex { position: [3.0, 5.0] }));
    }

    #[test]
    fn test_handle_key() {
        assert_eq!(false, handle_key(Keycode::Escape));
        assert_eq!(true, handle_key(Keycode::Space));
    }
}
