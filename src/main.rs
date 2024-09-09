use freya::prelude::*;
pub mod solace_app;
pub mod backend;

fn main() {
    let mut test_server = backend::Server::construct("ZavisSMP", 25565);
    test_server.init();
    test_server.start_server();
    let toml = toml::to_string(&test_server).unwrap();
    println!("{}", toml);
    //launch(solace_app::_app);
}