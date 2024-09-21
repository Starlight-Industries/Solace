use std::env;
use std::fs;

pub fn get_working_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap());
    println!("Home dir is:{}", home_dir);

    return home_dir;
}
pub fn get_server_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap());
    let server_dir = format!("{}/.solace/servers", home_dir);
    return server_dir;
}
pub fn get_server_list() -> Vec<String> {
    let working_dir = get_server_dir();
    let paths = fs::read_dir(working_dir).unwrap();
    let mut server_list = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        println!("Name: {}", path.display());
        if let Some(path_str) = path.file_name() {
            // Convert the PathBuf to a string
            server_list.push(path_str.to_str().unwrap().to_string()); // Push the string version of the path into the Vec
        }
    }
    if !server_list.is_empty() {
        server_list
    } else {
        panic!()
    }
}
