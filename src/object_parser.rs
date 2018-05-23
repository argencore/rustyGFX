#[path = "matrix_math_helper.rs"]
mod matrix_math_helper;
use std::fs::File;
use std::io::prelude::*;

pub fn vertices(fileName :String)-> Vec<matrix_math_helper::Vertex>{
    let vertices :Vec<matrix_math_helper::Vertex> = Vec::new();
    //read file
    //get all lines with v and vt
    let vertStrings = fileSection(fileName,"v".to_string());
    let texStrings = fileSelection(fileName,"vt".to_string());
    //parse those into new vertex structs make sure not out of bounds
    //add those to vector
    return vertices
}
pub fn normals()->Vec<matrix_math_helper::Normal>{
    let normals :Vec<matrix_math_helper::Normal> = Vec::new();
    //read file
    //get all lines with vn
    //parse into new normal struct
    //add to vector
    return normals
}
pub fn indices()->Vec<i32>{
    let indices :Vec<i32> = Vec::new();
    //read file
    //get all lines with f
    //figure out what to do with it
    return indices
}

pub fn fileSection(fileName :String, token :String) -> Vec<String>{
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