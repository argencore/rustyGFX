#[macro_use]
extern crate glium;
extern crate image;

#[path = ".\\window.rs"]
mod window;

use glium::glutin;
use glium::Surface;

///this is an enume to define the state of the
//program at any point it is useful for code flow 
//control
#[derive(Copy,Clone,PartialOrd,PartialEq,Eq)]
enum program_state{
    INITIALIZING,
    INITIALIZED,
    RUNNING,
    STOPPING,
    STOPPED,
    CLOSEING,

}

fn main() {
    //make an event handler
    let mut events_loop = glutin::EventsLoop::new();
    //set state to initializing
    let mut state = program_state::INITIALIZING;
    //do all of the initialization which returns a window/display
    let window = init(&events_loop);
    //update state
    state = program_state::INITIALIZED;
    //run the program passing in state the window and event loop since run
    //will be taking full control at this point.
    run(state, &window, events_loop);

}

///does all initialization and sets up default parameters if nessasary
fn init(ref mut events_loop: &glutin::EventsLoop)->glium::Display{

    //calls the create_window function from window.rs
    window::create_window(events_loop)

}

///main program loop that controls the program while running
fn run(mut state: program_state, ref window :&glium::Display, mut events_loop: glutin::EventsLoop)->(){
    //update state
    state = program_state::RUNNING;
    //while we should be running
    while state == program_state::RUNNING{
        //draw the scene
        draw(window);
        //handle events using pattern matching
        events_loop.poll_events(|ev|{
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::Closed => state = program_state::CLOSEING,
                    _=> (),
                },
                _=>(),
            }
        });
    }

}

fn draw(window :&glium::Display)->(){
    //get the draw target
    let mut target = window.draw();
    target.clear_color_and_depth((0.0,0.0,1.0,1.0),1.0);
    //finish and present window
    target.finish().unwrap();
}
