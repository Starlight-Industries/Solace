use std::env;
use std::fs;

pub fn get_working_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap());
    println!("Home dir is:{}", home_dir);

    return home_dir;
}
pub fn get_server_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap());
    let server_dir = format!("{}./solace/servers", home_dir);
    return server_dir;
}
pub fn get_server_list() {
    let working_dir = get_server_dir();
    let paths = fs::read_dir(working_dir).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
