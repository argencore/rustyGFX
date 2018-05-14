use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;

fn read_file(file_path :Path)-> String{
    let mut file = File::open(file_path.as_os_str())?;
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents)?;
    return contents
}