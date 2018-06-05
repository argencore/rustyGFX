// Copyright <2018> <Anthony Comstock>
use std::fs::File;
use std::io::prelude::*;

/// function name: read_file
/// inputs: path to the file to read as a string
/// outputs: string containing the contents of the file
/// description: this is a simple function that reads a file
/// and returns its contents as a string
pub fn read_file(file_path: &String) -> String {
    //open the file
    let mut file = File::open(file_path).expect("unable to open file");
    let mut contents = String::new();
    //read it into string
    file.read_to_string(&mut contents);
    return contents;
}
