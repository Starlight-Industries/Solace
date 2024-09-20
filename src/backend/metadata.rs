use std::env;

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
