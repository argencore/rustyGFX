// Copyright <2018> <Anthony Comstock>
#[macro_use]
extern crate glium;
extern crate image;

#[path = "window.rs"]
mod window;
#[path = "shader_parser.rs"]
mod shader_parser;
#[path = "matrix_math_helper.rs"]
pub mod mat;
#[path = "object_parser.rs"]
mod object_parser;
#[path = "camera.rs"]
mod camera;

use glium::glutin;
use glium::Surface;

///this is an enume to define the state of the
///program at any point it is useful for code flow 
///control
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

/// function name: init
/// inputs: mutable reference to the events_loop
/// outputs: glium::Display to be used as user window
/// description: this function initializes the window that the
/// user will interact with
fn init(ref mut events_loop: &glutin::EventsLoop)->glium::Display{

    //calls the create_window function from window.rs
    window::create_window(events_loop)

}

///main program loop that controls the program while running
/// function name: run
/// inputs: program State, window as reference to glium::Display, mutable EventsLoop
/// outputs: unit
/// description: this function takes care of event handing as well as calling draw and populating the
/// shaders and object
fn run(mut state: program_state,ref window :&glium::Display, mut events_loop: glutin::EventsLoop)->(){
    // update state
    state = program_state::RUNNING;
    // starting not in menu
    let mut in_menu = false;

    // init state of the viewer camera
    let mut position = [2.0,1.0,1.0];
    let mut direction = [-2.0,-1.0,1.0];

    // init index into the lists of objects to be loaded on demand
    let mut index = 0;
    // get the object from the file
    let mut objectName = object_parser::getObjectFileName(index);

    // load in the vertex and fragment shader from file
    let mut vertex_shader_src = shader_parser::read_file(&"vertex_shader.vert".to_string());
    let mut fragment_shader_src = shader_parser::read_file(&"fragment_shader.frag".to_string());

    // get the object data from the file into a usable format for openGL
    let mut shape = object_parser::positions(objectName.clone());
    let mut texels = object_parser::texels(objectName.clone());
    let mut normals = object_parser::normals(objectName.clone());
    let mut posTex = object_parser::posTex(&shape,&texels);
    //while we should be running
    while state == program_state::RUNNING{
    // create the shader program from the files loaded
    let program = glium::Program::from_source(*window, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        //draw the scene
        draw(window,program,&posTex,&normals, &position, &direction);
        //handle events using pattern matching
        events_loop.poll_events(|ev|{
            match ev {
                // match on WindowEvent
                glutin::Event::WindowEvent {event, ..} => match event {
                    // if window event is closed set state to closing to end loop
                    glutin::WindowEvent::Closed => state = program_state::CLOSEING,
                    // match on KeyboardInput
                    glutin::WindowEvent::KeyboardInput{input,..} =>{
                        match input.virtual_keycode{
                            // if enter is hit set in_menu to true
                            Some(glutin::VirtualKeyCode::Return) => in_menu = true,
                            // while in_menu if v is hit reload vertex shader and exit menu
                            Some(glutin::VirtualKeyCode::V) => if in_menu {
                                vertex_shader_src = shader_parser::read_file(&"vertex_shader.vert".to_string());
                                in_menu = false;
                            },
                            // while in_menu if f is hit reload fragment shader and exit menu
                            Some(glutin::VirtualKeyCode::F) => if in_menu{
                                fragment_shader_src = shader_parser::read_file(&"fragment_shader.frag".to_string());
                                in_menu = false;
                            }
                            // while in_menu if o is hit reload file containing object names and get the first object
                            Some(glutin::VirtualKeyCode::O) => if in_menu{
                                index = 0;
                                objectName = object_parser::getObjectFileName(index);
                                shape = object_parser::positions(objectName.clone());
                                texels = object_parser::texels(objectName.clone());
                                normals = object_parser::normals(objectName.clone());
                                posTex = object_parser::posTex(&shape,&texels);
                                in_menu = false;
                            }
                            // while in_menu if n is hit get the next object from the file of names
                            Some(glutin::VirtualKeyCode::N) => if in_menu{
                                index+=1;
                                objectName = object_parser::getObjectFileName(index);
                                shape = object_parser::positions(objectName.clone());
                                texels = object_parser::texels(objectName.clone());
                                normals = object_parser::normals(objectName.clone());
                                posTex = object_parser::posTex(&shape,&texels);
                                in_menu = false;
                            }
                            // the following are mappings from keys to camera position and direction allowing for movement in the application
                            Some(glutin::VirtualKeyCode::W) => position[2] = position[2] + 0.1,
                            Some(glutin::VirtualKeyCode::A) => position[0] = position[0] - 0.1,
                            Some(glutin::VirtualKeyCode::S) => position[2] = position[2] - 0.1,
                            Some(glutin::VirtualKeyCode::D) => position[0] = position[0] + 0.1,
                            Some(glutin::VirtualKeyCode::Q) => direction[0] = direction[0] - 0.1,
                            Some(glutin::VirtualKeyCode::E) => direction[0] = direction[0] + 0.1,
                            Some(glutin::VirtualKeyCode::Up) => direction[1] = direction[1] + 0.1,
                            Some(glutin::VirtualKeyCode::Down) => direction[1] = direction[1] - 0.1,
                            Some(glutin::VirtualKeyCode::Right) => direction[2] = direction[2] + 0.1,
                            Some(glutin::VirtualKeyCode::Left) => direction[2] = direction[2] - 0.1,
                            Some(glutin::VirtualKeyCode::PageUp) => position[1] = position[1] + 0.1,
                            Some(glutin::VirtualKeyCode::PageDown) => position[1] = position[1] - 0.1,
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

/// function name: draw
/// inputs: window as glium::Display, shader program as glium::Program, position and texture as vector of PosTex, normals as vector of Normal, camera position as [f32;3], camera direction as [f32;3]
/// outputs: unit
/// description: this function makes the calls to render the scene as well as sets up the perspective for viewing it
fn draw(window :&glium::Display, program :glium::Program, posTex :&Vec<object_parser::mat::PosTex>, normals :&Vec<object_parser::mat::Normal>, position :&[f32;3], direction :&[f32; 3])->(){
    //get the draw target
    let mut target = window.draw();


    // set up the perspective matrix
     let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();
            [
                [f * aspect_ratio, 0.0,    0.0,                     0.0],
                [0.0,               f,     0.0,                     0.0],
                [0.0,               0.0, (zfar+znear)/(zfar-znear), 1.0],
                [0.0,               0.0, -(2.0*zfar*znear)/(zfar-znear),0.0],
            ]//this is what is returned from this struct(the matrix)
        };

        // set the uniforms that can be used by the shaders
        let uniforms = uniform! {
            perspective: perspective,
            view: camera::view_matrix(position,direction,&[0.0,1.0,0.0]),
            model: [
                [0.4,0.0,0.0,0.0],
                [0.0,0.4,0.0,0.0],
                [0.0,0.0,0.4,0.0],
                [0.0,0.0,2.0,1.0f32],
            ],
            u_light: [-1.0,0.4,0.9f32],
        };

        // set up openGL to use Depth for 3D images
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

    // set up the vertex, normal, and index buffers
    let vertex_buffer = glium::VertexBuffer::new(window, posTex).unwrap();
    let normal_buffer = glium::VertexBuffer::new(window, normals).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //clear the background to black
    target.clear_color_and_depth((0.0,0.0,0.0,1.0),1.0);
    //draw the scene
    target.draw((&vertex_buffer,&normal_buffer),&indices,&program,&uniforms,&params).unwrap();
    //finish and present window
    target.finish().unwrap();
}

