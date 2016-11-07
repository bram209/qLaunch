extern crate ini;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate itertools;
extern crate gdk_sys;
extern crate gtk_sys;
extern crate gdk_pixbuf;

use std::str;

//modules
mod engine;
mod execution;
mod applications;
mod gui;
mod icons;
mod utils;

use engine::ApplicationSearcher;

fn main() {
    let engine = ApplicationSearcher::new();
    gui::create_and_setup_gui(engine);
}
