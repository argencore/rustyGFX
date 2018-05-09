
#[path = ".\\matrix_math_helper.rs"]
mod matrix_math_helper;

use glium::glutin;
use glium;

pub fn create_window(ref mut events_loop :&glutin::EventsLoop) -> glium::Display{

    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    glium::Display::new(window, context, &events_loop).unwrap()
}