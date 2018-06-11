//! Copyright <2018> <Anthony Comstock>
//! [This program is licensed under the "Apache Licence"]
//! Please see the file LICENSE in the source distribution of this software for license terms.

//! the window file creates and initializes the window that will be drawn in by the program

#[path = "matrix_math_helper.rs"]
mod matrix_math_helper;

use glium::glutin;
use glium;

///function to create the window and return it
pub fn create_window(ref mut events_loop: &glutin::EventsLoop) -> glium::Display {
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    glium::Display::new(window, context, &events_loop).unwrap()
}
