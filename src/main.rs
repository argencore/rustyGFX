#[macro_use]
extern crate glium;
extern crate image;

#[path = ".\\window.rs"]
mod window;

use glium::glutin;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    init(&events_loop);
    run();

}

fn init(ref mut events_loop: &glutin::EventsLoop)->(){
    let display = window::create_window(events_loop);

}

fn run()->(){

}
