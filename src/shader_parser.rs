use std::fs::File;
use std::io::prelude::*;
/*
#[derive(Debug,Clone,Copy)]
pub enum Shader {
    VERTEX,
    PIXEL,
    GEOMETRY,
    TESELATION,
    TESELATION_CONTROL,
    COMPUTE,
    ERROR,
}
*/
pub fn read_file(file_path :&String)-> String{
    let mut file = File::open(file_path).expect("unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return contents
}

/*
pub fn shader_parser(file_path :String)->(Shader,String){
    let extension = &file_path[file_path.len()-5 .. file_path.len()];
    match extension{
        ".vert" => return (Shader::VERTEX,read_file(&file_path)),
        ".frag" => return (Shader::PIXEL,read_file(&file_path)),
        ".tesc" => return (Shader::TESELATION_CONTROL,read_file(&file_path)),
        ".tese" => return (Shader::TESELATION,read_file(&file_path)),
        ".geom" => return (Shader::GEOMETRY,read_file(&file_path)),
        ".comp" => return (Shader::COMPUTE,read_file(&file_path)),
        _ => return (Shader::ERROR,"not a correct file extension".to_string()),

    }

}

#[test]
fn test_read_file(){
    let mut contents = read_file(&("test.vert".to_string()));
    println!("{}",contents);
}

#[test]
fn test_shader_parser(){
    shader_parser("test.vert".to_string());
    shader_parser("test.frag".to_string());
    shader_parser("test.comp".to_string());
    shader_parser("test.geom".to_string());
    shader_parser("test.tesc".to_string());
    shader_parser("test.tese".to_string());
}

*/