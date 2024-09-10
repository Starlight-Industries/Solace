use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use freya::prelude::*;
use serde::Serialize;
pub mod solace_app;
pub mod backend;
use colored::{self, Colorize};
fn main() {
    let mut test_server = backend::Server::construct("TestSMP", 25565);
    if !test_server.is_initalized() {
        test_server.init()
    }
    //println!("{}", toml);
    let _ = test_server.start_server();

    //launch(solace_app::_app);
}