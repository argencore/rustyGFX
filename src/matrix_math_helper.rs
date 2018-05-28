// Copyright <2018> <Anthony Comstock>
//the following definitions are taken from or influenced by the glium examples since they are the format needed for glium



/// struct name: Vertex
/// description: a structure to hold
/// the position of a point in 3D space
#[derive(Copy,Clone,Debug)]
pub struct Vertex{
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

/// struct name: Texture
/// description: a structure to hold
/// the texture coordinates for a 3D
/// object
#[derive(Copy, Clone,Debug)]
pub struct Texture{
    pub tex_coords: [f32; 2],
}

implement_vertex!(Texture, tex_coords);

/// struct name: Normal
/// description: a structure to hold
/// the normal data for a face of a 3D
/// object
#[derive(Copy, Clone,Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

/// struct name: Face
/// description: a structure to hold 
/// the face data that is recieved from
/// parsing obj files
#[derive(Copy, Clone, Debug)]
pub struct Face{
    pub position: u16,
    pub tex_coord: u16,
    pub normal: u16,
}

implement_vertex!(Face, position,tex_coord,normal);

/// struct name: PosTex
/// description: a structure
/// to hold the positon and 
/// texture coordinates of a Vertex
#[derive(Copy, Clone, Debug)]
pub struct PosTex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(PosTex,position,tex_coords);



