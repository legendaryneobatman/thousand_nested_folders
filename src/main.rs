use std::env;
use std::fs;

fn main() {
    const FOLDER_LEVELS: i16 = 1000;
    
    let current_directory_path =  env::current_dir().unwrap().to_str().unwrap().to_string();
    let mut addition_path  = String::new();

    for index in 0..FOLDER_LEVELS {
        addition_path.push_str("\\");
        addition_path.push_str(&*index.to_string());
    }
    
    let result = current_directory_path + &*addition_path;
    
    
    fs::create_dir_all(result).unwrap()
}
