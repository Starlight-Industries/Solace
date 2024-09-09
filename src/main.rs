use freya::prelude::*;
pub mod solace_app;
pub mod backend;

fn main() {
    let mut test_server = backend::Server::construct("Godot cafe", 25565);
    test_server.init();
    test_server.start_server();
    //launch(solace_app::_app);
}