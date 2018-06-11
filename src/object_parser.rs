// Copyright <2018> <Anthony Comstock>
#[path = "matrix_math_helper.rs"]
pub mod mat;
use std::fs::File;
use std::io::prelude::*;

/// struct name: Model
/// description: this struct is designed
/// to hold the meta data about a model being used
/// by the program it holds the number of vertices,
/// positions, texels, normals, and faces of the model
#[derive(Copy, Clone, Debug)]
pub struct Model {
    pub vertices: isize,
    pub positions: isize,
    pub texels: isize,
    pub normals: isize,
    pub faces: isize,
}

/// function name: vertices
/// inputs: the file name as a string
/// outputs: a vector of Vertex structures
/// description: this function takes the name of a file
/// gets the section of the file related to vertices
/// and parses that section into a series of Vertex structures that
// it adds to the vector that it will return
pub fn vertices(fileName: &String) -> Vec<mat::Vertex> {
    let mut vertices: Vec<mat::Vertex> = Vec::new();
    //read file
    //get all lines with v
    let vertStrings = fileSection(&fileName, "v".to_string());
    //for each line in the block that needs to be parsed
    for string in vertStrings {
        // create a vertex structure to populate
        let mut vertex: mat::Vertex = mat::Vertex {
            position: [0.0, 0.0, 0.0],
        };
        // create a vector of words by splitting on the space
        let tmp: Vec<&str> = string.split(' ').collect();
        // match the length of the vector to know if it is a 1D, 2D, or 3D object
        match tmp.len() {
            // turn each of the words that are not the starting v into f32 floats and add them to the vertex and vector
            4 => {
                vertex.position = [
                    tmp[1].parse::<f32>().unwrap(),
                    tmp[2].parse::<f32>().unwrap(),
                    tmp[3].parse::<f32>().unwrap(),
                ];
                vertices.push(vertex);
            }
            3 => {
                vertex.position = [
                    tmp[1].parse::<f32>().unwrap(),
                    tmp[2].parse::<f32>().unwrap(),
                    0.0,
                ];
                vertices.push(vertex);
            }
            2 => {
                vertex.position = [tmp[1].parse::<f32>().unwrap(), 0.0, 0.0];
                vertices.push(vertex);
            }
            _ => println!("something went wrong trying to fill the vertices structure"),
        }
    }
    return vertices;
}

/// function name: unmodifiedTextures
/// input: name of the file as a string
/// output: a vector of the Texture struct
/// description: this function takes a file name
/// gets the section of that file related to texture data
/// parses that into Texture structures
/// adds those to a vector and returns that
pub fn unmodifiedTextures(fileName: &String) -> Vec<mat::Texture> {
    // create vector of Texture
    let mut textures: Vec<mat::Texture> = Vec::new();

    // get relevant piece of file
    let texStrings = fileSection(&fileName, "vt".to_string());

    // for each line in the piece of file
    for string in texStrings {
        // create a Texture
        let mut texture: mat::Texture = mat::Texture {
            tex_coords: [0.0, 0.0],
        };
        //split and parse the string into the Texture
        let tmp: Vec<&str> = string.split(' ').collect();
        if tmp.len() >= 3 {
            texture.tex_coords = [
                tmp[1].parse::<f32>().unwrap(),
                tmp[2].parse::<f32>().unwrap(),
            ];
        }
        //add it to the vector
        textures.push(texture);
    }
    return textures;
}

/// function name: unmodifiedNormals
/// inputs: the name of the file to parse as a string
/// outputs: a vector of the Normal struct
/// description: this function takes the piece of a file
/// related to normals and parses it into a vector of normal
/// structs
pub fn unmodifiedNormals(fileName: &String) -> Vec<mat::Normal> {
    // create vector of structs
    let mut normals: Vec<mat::Normal> = Vec::new();
    // get chunk of file
    let normalStrings = fileSection(&fileName, "vn".to_string());
    // parse those into new vertex structs make sure not out of bounds
    for string in normalStrings {
        //for each line creat and populate a Normal struct and add it to the vector
        let mut normal: mat::Normal = mat::Normal {
            normal: (0.0, 0.0, 0.0),
        };
        let tmp: Vec<&str> = string.split(' ').collect();
        match tmp.len() {
            4 => {
                normal.normal = (
                    tmp[1].parse::<f32>().unwrap(),
                    tmp[2].parse::<f32>().unwrap(),
                    tmp[3].parse::<f32>().unwrap(),
                );
                normals.push(normal);
            }
            3 => {
                normal.normal = (
                    tmp[1].parse::<f32>().unwrap(),
                    tmp[2].parse::<f32>().unwrap(),
                    0.0,
                );
                normals.push(normal);
            }
            2 => {
                normal.normal = (tmp[1].parse::<f32>().unwrap(), 0.0, 0.0);
                normals.push(normal);
            }
            _ => println!("something went wrong trying to fill the vertices structure"),
        }
    }
    return normals;
}

/// function name: faces
/// inputs: the name of the file as a string
/// outputs: a vector of the Face struct
/// description: this function takes in a filename
/// it get the piece of the file related to face data
/// it then parses that into a vector of Face structs
pub fn faces(fileName: &String) -> Vec<mat::Face> {
    // create vector
    let mut faces: Vec<mat::Face> = Vec::new();
    // get all lines with f
    let faceStrings = fileSection(&fileName, "f".to_string());

    for line in faceStrings {
        //split the lines into words
        for word in line.split(' ') {
            // create face to add to vec
            let mut face: mat::Face = mat::Face {
                position: 0,
                tex_coord: 0,
                normal: 0,
            };
            if word != "f" {
                // split on / to get values of word by themselves
                let tmp: Vec<&str> = word.split('/').collect();
                // if all three fields are full
                if tmp.len() >= 3 {
                    //parse as u16
                    face.position = tmp[0].parse::<u16>().unwrap();
                    face.tex_coord = tmp[1].parse::<u16>().unwrap();
                    face.normal = tmp[2].parse::<u16>().unwrap();
                } else {
                    // if all three fields are not full set texture to 0
                    face.position = tmp[0].parse::<u16>().unwrap();
                    face.tex_coord = 0;
                    face.normal = tmp[1].parse::<u16>().unwrap();
                }
                faces.push(face);
            }
        }
    }
    return faces;
}

/// function name: positions
/// inputs: the name of the file as a string
/// outputs: a vector of Vertex structs
/// description: this function takes in a file name
/// it runs vertices and faces to get vectors of vertices and
/// faces and uses those to create a new ordering for the vertices
/// that is acceptable for openGL to use in rendering
pub fn positions(fileName: String) -> Vec<mat::Vertex> {
    // create vector of positions
    let mut positions: Vec<mat::Vertex> = Vec::new();
    // get vertices
    let verts = vertices(&fileName);
    // get faces
    let mut indexes = faces(&fileName);
    // flip the face vector so that pop will remove them in the order they were put in.
    indexes.reverse();
    // while there are indices which is the first part of the face data
    while !indexes.is_empty() {
        // get the index
        let i = indexes.pop().unwrap();

        //push the vertices that relate to the indexes - 1 for 0 indexing
        positions.push(verts[(i.position - 1) as usize]);
    }
    return positions;
}

/// function name: texels
/// inputs: the file name as a string
/// outputs: a vector of Textures
/// description: this function gets and reorders
/// the textures based on the face data so that they can
/// be used by openGL
pub fn texels(fileName: String) -> Vec<mat::Texture> {
    // create Texture vector
    let mut texels: Vec<mat::Texture> = Vec::new();
    // get texture data
    let tex = unmodifiedTextures(&fileName);
    // get index data
    let mut indexes = faces(&fileName);
    indexes.reverse();
    while !indexes.is_empty() {
        let i = indexes.pop().unwrap();
        //reorder texture data by index
        texels.push(tex[(i.tex_coord - 1) as usize])
    }
    return texels;
}

/// function name: normals
/// inputs: the file name as a string
/// outputs: a vector of Normal structs
/// description: this function gets the normal data
/// from a file and reorders it by index for openGL
pub fn normals(fileName: String) -> Vec<mat::Normal> {
    // create vector
    let mut normals: Vec<mat::Normal> = Vec::new();
    // get normal data
    let norm = unmodifiedNormals(&fileName);
    // get index data
    let mut indexes = faces(&fileName);
    indexes.reverse();
    while !indexes.is_empty() {
        let i = indexes.pop().unwrap();
        //reorder data by index
        normals.push(norm[(i.normal - 1) as usize])
    }
    return normals;
}

/// function name: posTex
/// inputs: positions as a vector of Vertex structs, texels as a vector of Texture structs
/// outputs: a vector of PosTex structs
/// description: this function combines the position and texture data into a single list of data
pub fn posTex(positions: &Vec<mat::Vertex>, texels: &Vec<mat::Texture>) -> Vec<mat::PosTex> {
    // create vector
    let mut posTex: Vec<mat::PosTex> = Vec::new();
    // for each position
    for i in 0..positions.len() {
        let pos = positions[i];
        let tex = texels[i];
        // create a new PosTex and fill it with position and texture data
        let tmp: mat::PosTex = mat::PosTex {
            position: pos.position,
            tex_coords: tex.tex_coords,
        };
        // add it to the vector
        posTex.push(tmp);
    }
    return posTex;
}

///test that a posTex made two different ways comes out the same
#[test]
fn test_posTex(){
    let test_vertex = mat::Vertex{ position: [1.0,0.5,0.25]};
    let test_texture = mat::Texture{ tex_coords: [0.66,0.33]};
    let test_posTex = mat::PosTex{ position: [1.0,0.5,0.25], tex_coords: [0.66,0.33]};
    let mut vVec: Vec<mat::Vertex> = Vec::new();
    let mut tVec: Vec<mat::Texture> = Vec::new();
    vVec.push(test_vertex);
    tVec.push(test_texture);
    let test_posTex_vec = posTex(&vVec,&tVec);
    assert!(test_posTex_vec[0].position == test_posTex.position && test_posTex_vec[0].tex_coords == test_posTex.tex_coords);
}

/// function name: getObjectFileName
/// inputs: the index into the list of names as usize
/// outputs: the string that is the file name
/// description: this function reads in the file that contains
/// the names of the object files and then returns the one at the index
/// or modulus of the index to avoid out of bounds
pub fn getObjectFileName(index: usize) -> String {
    // get the file of names
    let mut file = File::open("objectToLoad.txt").expect("unable to open objectToLoad.txt");
    // get the content as a string
    let mut content = String::new();
    file.read_to_string(&mut content);
    //get the lines of the file
    let lines: Vec<&str> = content.lines().collect();
    //return the line that is at the index % number of file names
    return lines[index % lines.len()].to_string().trim().to_string();
}

/// function name: fileSection
/// inputs: name of file as string, token to look for as start of line in the file
/// outputs: a vector of strings that are the lines in the file related to the token
/// description: this function gets all of the lines of a file that start with the token
/// passed to it.
pub fn fileSection(ref fileName: &String, token: String) -> Vec<String> {
    // get file
    let mut file = File::open(fileName).expect("unable to open file");
    let mut content = String::new();
    // create vector of strings for lines
    let mut tokenLines: Vec<String> = Vec::new();
    // get file content
    file.read_to_string(&mut content);
    // for each line in the file
    for line in content.lines() {
        // check if the first word matches the token and if so add it to the vector
        let v: Vec<&str> = line.split(' ').collect();
        if v[0] == &token {
            tokenLines.push(line.to_string());
        }
    }
    return tokenLines;
}

///test fileSection with the test file which should not change
#[test]
fn test_fileSelection(){
    let test_section = fileSection(&"testfile.txt".to_string(),"TEST".to_string());
    assert!(test_section[0] == "TEST this is a test file!".to_string());
}
