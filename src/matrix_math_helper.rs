//the following definitions for vertex and normal structs are taken from the glium example teapot.


///basic vertex struct
#[derive(Copy,Clone,Debug)]
pub struct Vertex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

//basic normal structure
#[derive(Copy, Clone,Debug)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);




