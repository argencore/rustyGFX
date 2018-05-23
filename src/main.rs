#[macro_use]
extern crate glium;
extern crate image;

#[path = "window.rs"]
mod window;
#[path = "shader_parser.rs"]
mod shader_parser;
#[path = "matrix_math_helper.rs"]
mod mat;
#[path = "object_parser.rs"]
mod object_parser;

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
fn run(mut state: program_state,ref window :&glium::Display, mut events_loop: glutin::EventsLoop)->(){
    //update state
    state = program_state::RUNNING;
    let mut in_menu = false;
    let mut vertex_shader_src = shader_parser::read_file(&"vertex_shader.vert".to_string());
    let mut fragment_shader_src = shader_parser::read_file(&"fragment_shader.frag".to_string());
    let mut vertStrings = object_parser::fileSection("object.obj".to_string(),"v".to_string());
    for string in vertStrings{
        println!("{}",string);
    }
    //while we should be running
    while state == program_state::RUNNING{
    let program = glium::Program::from_source(*window, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        //draw the scene
        draw(window,program);
        //handle events using pattern matching
        events_loop.poll_events(|ev|{
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::Closed => state = program_state::CLOSEING,
                    glutin::WindowEvent::KeyboardInput{input,..} =>{
                        match input.virtual_keycode{
                            Some(glutin::VirtualKeyCode::Return) => in_menu = true,
                            Some(glutin::VirtualKeyCode::V) => if in_menu {
                                vertex_shader_src = shader_parser::read_file(&"vertex_shader.vert".to_string());
                                in_menu = false;
                            },
                            Some(glutin::VirtualKeyCode::F) => if in_menu{
                                fragment_shader_src = shader_parser::read_file(&"fragment_shader.frag".to_string());
                                in_menu = false;
                            }
                            _=>(),
                        }

                    },
                    _=> (),
                },
                _=>(),
            }
        });
    }

}

fn draw(window :&glium::Display, program :glium::Program)->(){
    //get the draw target
    let mut target = window.draw();
    let vertex1 = mat::Vertex { position: [-0.5, -0.5,0.0], tex_coords: [0.0,0.0] };
    let vertex2 = mat::Vertex { position: [ 0.0,  0.5,0.0] , tex_coords: [0.0,0.0]};
    let vertex3 = mat::Vertex { position: [ 0.5, -0.25,0.0] , tex_coords: [0.0,0.0]};
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    target.clear_color_and_depth((0.0,0.0,1.0,1.0),1.0);
    target.draw(&vertex_buffer,&indices,&program,&glium::uniforms::EmptyUniforms,&Default::default()).unwrap();
    //finish and present window
    target.finish().unwrap();
}
