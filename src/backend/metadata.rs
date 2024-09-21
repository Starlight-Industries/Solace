use std::env;
use std::fs;
use std::path::Path;

pub fn get_working_dir() -> String {
    let binding = env::home_dir().expect("Unable to find home dir!");
    let home_dir = binding.display().to_string();

    return home_dir;
}
pub fn get_server_dir() -> String {
    let binding = env::home_dir().expect("Unable to find home dir!");
    let home_dir = binding.display();
    let server_dir = format!("{}/.solace/servers", home_dir);
    return server_dir;
}
pub fn get_server_list() -> Vec<String> {
    let working_dir = get_server_dir();
    let working_path = Path::new(working_dir.as_str());
    let mut server_list = Vec::new();
    if !working_path.exists() {
        fs::create_dir_all(working_path).unwrap();
    }
    let paths = fs::read_dir(working_dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        println!("Name: {}", path.display());
        if let Some(path_str) = path.file_name() {
            server_list.push(path_str.to_str().unwrap().to_string());
        }
    }
    if !server_list.is_empty() {
        server_list
    } else {
        return vec!("none".to_string())
    }
}
