

pub mod solace_app;
pub mod backend;
fn main() {
    let mut test_server = backend::Server::construct("TestServer", 25565);
    if test_server.is_initalized() != true {
        test_server.init()
    } else {
        //std::process::exit(0)
    }
    //println!("{}", toml);
    let _ = test_server.start_server();

    //launch(solace_app::_app);
}