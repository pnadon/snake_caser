use std::{env, fs, path::Path};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Incorrect number of parameters: specify a single path");
    }
    for entry in args[1..].iter() {
        let path = Path::new(entry);
        if !path.exists() {
            println!("{:?} is not a file", path);
        }
        
        let parent = path.parent()
            .and_then(|s| Some(s))
            .unwrap_or(Path::new(""));

        let extension = path.extension()
            .and_then(|s| Some(".".to_owned()+ s.to_str().unwrap()).to_owned())
            .unwrap_or("".to_owned());

        let snake_path = to_snake(path.file_stem().expect("file has no file name"));

        let final_name = parent.join(snake_path + &extension);
        println!("Renaming {:?} to {:?}", path, final_name);
        fs::rename(path.clone(), final_name).expect("Failed to rename file");
    }
}

fn to_snake(path: &std::ffi::OsStr) -> String {
    path.to_string_lossy().chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '_')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("_")
        .to_lowercase()
        
}
