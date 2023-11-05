#![windows_subsystem = "windows"]
mod raylib;
mod fltk;

fn main() {
    raylib::config::init_folder();
    fltk::main::run_app();
}
