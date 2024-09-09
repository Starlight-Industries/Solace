use freya::prelude::*;
pub mod solace_app;
pub mod backend;

fn main() {
    let mut test_server = backend::Server::construct("TestSMP", 25565);
    test_server.init();
    let toml = toml::to_string(&test_server).unwrap();
    println!("{}", toml);
    let _ = test_server.start_server();
    //launch(solace_app::_app);
}