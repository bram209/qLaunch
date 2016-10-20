extern crate ini;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate itertools;
extern crate gdk_sys;

use std::env;


mod engine;
mod execution;
mod applications;
mod gui;

use engine::SearchResult;
use engine::SearchEngine;
use engine::ApplicationSearcher;

fn main() {
    let engine = ApplicationSearcher::new();
    gui::create_and_setup_gui(engine);
}
