// build.rs

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("build-info/icon.ico");
    res.compile().unwrap();
}

