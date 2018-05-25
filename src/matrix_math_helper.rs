//the following definitions for vertex and normal structs are taken from the glium example teapot.


///basic vertex struct
#[derive(Copy,Clone,Debug)]
pub struct Vertex{
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone,Debug)]
pub struct Texture{
    pub tex_coords: [f32; 2],
}

implement_vertex!(Texture, tex_coords);

//basic normal structure
#[derive(Copy, Clone,Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

#[derive(Copy, Clone, Debug)]
pub struct Face{
    pub position: u16,
    pub tex_coord: u16,
    pub normal: u16,
}

implement_vertex!(Face, position,tex_coord,normal);

#[derive(Copy, Clone, Debug)]
pub struct PosTex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(PosTex,position,tex_coords);



