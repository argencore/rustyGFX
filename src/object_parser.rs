#[path = "matrix_math_helper.rs"]
pub mod mat;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy,Clone,Debug)]
pub struct Model{
    pub vertices: isize,
    pub positions: isize,
    pub texels: isize,
    pub normals: isize,
    pub faces: isize,
}


pub fn vertices(fileName :&String)-> Vec<mat::Vertex>{
    let mut vertices :Vec<mat::Vertex> = Vec::new();
    //read file
    //get all lines with v 
    let vertStrings = fileSection(&fileName,"v".to_string());
    //parse those into new vertex structs make sure not out of bounds
    for string in vertStrings{
        let mut vertex :mat::Vertex = mat::Vertex { position: [0.0, 0.0,0.0]};
        let tmp :Vec<&str> = string.split(' ').collect();
        match tmp.len(){
            4 => {vertex.position = [tmp[1].parse::<f32>().unwrap(),tmp[2].parse::<f32>().unwrap(),tmp[3].parse::<f32>().unwrap()];
                  vertices.push(vertex);},
            3 => {vertex.position = [tmp[1].parse::<f32>().unwrap(),tmp[2].parse::<f32>().unwrap(),0.0];
                  vertices.push(vertex);},
            2 => {vertex.position = [tmp[1].parse::<f32>().unwrap(),0.0,0.0];
                  vertices.push(vertex);},
            _ => println!("something went wrong trying to fill the vertices structure"),
        } 

    }
    return vertices
}

pub fn unmodifiedTextures(fileName :&String)->Vec<mat::Texture>{
    let mut textures :Vec<mat::Texture> = Vec::new();

    let texStrings = fileSection(&fileName,"vt".to_string());

    for string in texStrings{
        let mut texture :mat::Texture = mat::Texture {tex_coords:[0.0,0.0]};
        let tmp :Vec<&str> = string.split(' ').collect();
        if tmp.len() >= 3{
            texture.tex_coords = [tmp[1].parse::<f32>().unwrap(),tmp[2].parse::<f32>().unwrap()];
        }
        textures.push(texture);
    }
    return textures
}

pub fn unmodifiedNormals(fileName :&String)->Vec<mat::Normal>{
    let mut normals :Vec<mat::Normal> = Vec::new();
    let normalStrings = fileSection(&fileName,"vn".to_string());
    //parse those into new vertex structs make sure not out of bounds
    for string in normalStrings{
        let mut normal :mat::Normal = mat::Normal { normal: (0.0, 0.0,0.0)};
        let tmp :Vec<&str> = string.split(' ').collect();
        match tmp.len(){
            4 => {normal.normal = (tmp[1].parse::<f32>().unwrap(),tmp[2].parse::<f32>().unwrap(),tmp[3].parse::<f32>().unwrap());
                  normals.push(normal);},
            3 => {normal.normal = (tmp[1].parse::<f32>().unwrap(),tmp[2].parse::<f32>().unwrap(),0.0);
                  normals.push(normal);},
            2 => {normal.normal = (tmp[1].parse::<f32>().unwrap(),0.0,0.0);
                  normals.push(normal);},
            _ => println!("something went wrong trying to fill the vertices structure"),
        } 

    }
    return normals
}
pub fn faces(fileName :&String)->Vec<mat::Face>{
    let mut faces :Vec<mat::Face> = Vec::new();
    //read file
    //get all lines with f
    let faceStrings = fileSection(&fileName,"f".to_string());

    //figure out what to do with it
    for line in faceStrings{
        for word in line.split(' '){
            let mut face :mat::Face = mat::Face {position: 0,tex_coord: 0,normal: 0};
            if word != "f"{
                let tmp :Vec<&str> = word.split('/').collect();
                if tmp.len() >= 3{
                    face.position = tmp[0].parse::<u16>().unwrap();
                    face.tex_coord = tmp[1].parse::<u16>().unwrap();
                    face.normal = tmp[2].parse::<u16>().unwrap();
                }else{

                    face.position = tmp[0].parse::<u16>().unwrap();
                    face.tex_coord = 0;
                    face.normal = tmp[1].parse::<u16>().unwrap();
                }
                faces.push(face);
            }
        }
    }
    return faces
}

pub fn positions(fileName :String)->Vec<mat::Vertex>{
    let mut positions :Vec<mat::Vertex> = Vec::new();
    let mut verts = vertices(&fileName);
    let mut indexes = faces(&fileName);
    indexes.reverse();
    while !indexes.is_empty() {
        let mut i = indexes.pop().unwrap();

        positions.push(verts[(i.position - 1) as usize]);


    }
    return positions
    
}

pub fn texels(fileName :String)->Vec<mat::Texture>{
    let mut texels :Vec<mat::Texture> = Vec::new();
    let mut tex = unmodifiedTextures(&fileName);
    let mut indexes = faces(&fileName);
    indexes.reverse();
    while !indexes.is_empty(){
        let mut i = indexes.pop().unwrap();

        texels.push(tex[(i.tex_coord - 1) as usize])
    }
    return texels
}

pub fn normals(fileName :String)->Vec<mat::Normal>{
    let mut normals :Vec<mat::Normal> = Vec::new();
    let mut norm = unmodifiedNormals(&fileName);
    let mut indexes = faces(&fileName);
    indexes.reverse();
    while !indexes.is_empty(){
        let mut i = indexes.pop().unwrap();

        normals.push(norm[(i.normal - 1) as usize])
    }
    return normals
}

pub fn posTex(positions :&Vec<mat::Vertex>,texels :&Vec<mat::Texture>)-> Vec<mat::PosTex>{
    let mut posTex :Vec<mat::PosTex> = Vec::new();
    for i in 0 .. positions.len(){
        let pos = positions[i];
        let tex = texels[i];
        let tmp :mat::PosTex = mat::PosTex{position: pos.position, tex_coords: tex.tex_coords};
        posTex.push(tmp);
    }
    return posTex;
}

pub fn getObjectFileName(index :usize)->String{
    let mut file = File::open("objectToLoad.txt").expect("unable to open objectToLoad.txt");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let lines :Vec<&str> = content.lines().collect();
    return lines[index % lines.len()].to_string().trim().to_string()
}

pub fn fileSection(ref fileName :&String, token :String) -> Vec<String>{
    let mut file = File::open(fileName).expect("unable to open file");
    let mut content = String::new();
    let mut tokenLines :Vec<String> = Vec::new();
    file.read_to_string(&mut content);
    for line in content.lines(){
        let v :Vec<&str> = line.split(' ').collect();
        if v[0] == &token {
            tokenLines.push(line.to_string());
        }
    }
    return tokenLines
}