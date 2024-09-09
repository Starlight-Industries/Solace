#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use solace::solace_app;
//use solace::solace_backend;

fn main() {
    launch(solace_app::_app);
}