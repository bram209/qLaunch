use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering, AtomicIsize};

use gtk;
use glib;
use gdk::enums::key;
use gdk_sys::GDK_KEY_PRESS;
use gtk::prelude::*;
use gtk::{ListStore, ScrolledWindow, Orientation, Button, Window, WindowType, SearchEntry, WindowPosition, TreeView, TreeViewColumn, TreePath, CellRendererText, Container};
use itertools::Itertools;

use engine::SearchEngine;
use engine::SearchResult;
use engine::ApplicationSearcher;
use execution;

pub fn create_and_setup_gui(search_engine: ApplicationSearcher) {
    env::set_var("GTK_THEME", "Adwaita:dark");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_position(WindowPosition::CenterAlways);
    window.set_title("App launcher");
    window.set_default_size(450, 400);
    window.set_decorated(false);
    window.set_resizable(false);

    //gui layout
    //  - box
    //      - search entry
    //      - scrolled window
    //          - treeview

    let box_container = gtk::Box::new(Orientation::Vertical, 0);
    let search_entry = SearchEntry::new();
    let scrolled_window = ScrolledWindow::new(None, None);
    let treeview = TreeView::new();

    box_container.add(&search_entry);
    scrolled_window.add(&treeview);
    scrolled_window.set_vexpand(true);
    box_container.add(&scrolled_window);

    //setup list store
    let column_types = [glib::Type::String];
    let completion_store = ListStore::new(&column_types);
    treeview.set_model(Some(&completion_store));

    //setup treeview
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    treeview.append_column(&column);

    treeview.set_headers_visible(false);
    treeview.set_show_expanders(false);
    treeview.set_enable_search(false);
    treeview.set_activate_on_single_click(true);
    //    treeview.set_search_entry(Some(&search_entry));

    window.add(&box_container);
    window.show_all();
    WindowExt::set_opacity(&window, 0.9);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let current_search_results: Rc<RefCell<Vec<SearchResult>>> = Rc::new(RefCell::new(vec![]));

    let selected_result: Rc<RefCell<Option<SearchResult>>> = Rc::new(RefCell::new(None));
    let current_and_selected_result = (current_search_results.clone(), selected_result.clone());
    treeview.get_selection().connect_changed(move |tree_selection| {
        if let Some((completion_model, iter)) = tree_selection.get_selected() {
            if let Some(path) = completion_model.get_path(&iter) {
                let selected_number = path.get_indices()[0] as usize;
                let (ref current_results, ref selected_result) = current_and_selected_result;
                *selected_result.borrow_mut() = Some(current_results.borrow()[selected_number].clone());
            }
        }
    });

    let temp = selected_result.clone(); //ugly?
    treeview.connect_row_activated(move |treeview, path, column| {
        let selected = (*temp).borrow().clone();
        if let Some(search_result) = selected {
            execution::execute_result(&search_result);
            gtk::main_quit();
        }
    });

    search_entry.connect_activate(move |_| {
        let selected = selected_result.borrow().clone();
        if let Some(search_result) = selected {
            execution::execute_result(&search_result);
            gtk::main_quit();
        }
    });

    search_entry.connect_changed(move |s| {
        let text = s.get_buffer().get_text();
        completion_store.clear();
        if text.len() == 0 {
            if scrolled_window.is_visible() {
                scrolled_window.set_visible(false);
            }
            return
        }

        scrolled_window.set_visible(true);

        let results = search_engine.search(&text).collect_vec();
        if results.len() > 0 {
            for result in results.iter() {
                let iter = completion_store.append();
                completion_store.set(&iter, &[0], &[&result.name]);
            }

                * current_search_results.borrow_mut() = results;
            treeview.set_cursor(&TreePath::new_first(), None, false);
        }
    });


    window.connect_key_release_event(move |s, key| {
        let keyval = key.get_keyval();
        let keystate = key.get_state();
        // let keystate = (*key).state;
        println!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit(),
            _ => {}
        }

        Inhibit(false)
    });


    window.connect_focus_out_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}