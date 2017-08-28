use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering, AtomicIsize};

use gtk;
use glib;
use gdk::enums::key;
use gdk_sys::GDK_KEY_PRESS;
use gtk::prelude::*;

use gtk::{MessageDialog, ButtonsType, CellRendererPixbuf, CssProvider, CssProviderExt, StyleContext, ListStore,
    ScrolledWindow, Orientation, Button, Window, WindowType, SearchEntry, WindowPosition, TreeView,
    TreeViewColumn, TreePath, CellRendererText, Container, DIALOG_MODAL, MessageType};
use gdk_pixbuf::Pixbuf;
use gtk_sys;
use itertools::Itertools;
use std::mem;
use utils;

use engine::SearchEngine;
use engine::SearchResult;
use engine::ApplicationSearcher;
use execution;

pub fn create_and_setup_gui(search_engine: ApplicationSearcher) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_position(WindowPosition::Center);
    window.set_title("App launcher");
    window.set_decorated(false);
    window.set_default_size(450, 0);

    let css_path = format!("/usr/share/themes/{}/gtk-3.0/", get_gtk_theme().unwrap());
    let css_provider = CssProvider::new();
    // gtk::CssProviderExt::load_from_resource(&css_provider, format!("{}/gtk.gresource", css_path).as_ref());
    if css_provider.load_from_path(format!("{}/gtk-dark.css", css_path).as_ref()).is_err() {
        println!("unable to load CSS!\n{}", css_path);
    };

    let screen = window.get_screen().unwrap();
    StyleContext::add_provider_for_screen(&screen, &css_provider, 600);

    let box_container = gtk::Box::new(Orientation::Vertical, 0);
    let search_entry = SearchEntry::new();
    let scrolled_window = ScrolledWindow::new(None, None);
    let treeview = TreeView::new();

    //make sure that the search entry has no rounded corners
    let css_provider = CssProvider::new();
    gtk::CssProviderExt::load_from_data(&css_provider, "entry.search { border-radius: 0px; font-size: 22px; }".as_bytes());
    search_entry.get_style_context().unwrap().add_provider(&css_provider, 600);

    box_container.add(&search_entry);
    scrolled_window.add(&treeview);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_min_content_height(0);

    box_container.add(&scrolled_window);

    //setup list store
    let column_types = [Pixbuf::static_type(), String::static_type()];
    let completion_store = ListStore::new(&column_types);
    treeview.set_model(Some(&completion_store));

    //setup treeview
    let column = TreeViewColumn::new();
    let text_renderer = CellRendererText::new();
    let icon_renderer = CellRendererPixbuf::new();
    column.pack_start(&icon_renderer, false);
    column.add_attribute(&icon_renderer, "pixbuf", 0);
    column.pack_start(&text_renderer, true);
    column.add_attribute(&text_renderer, "text", 1);

    let image = Pixbuf::new_from_file(&(utils::qlauncher_settings_folder() + "resources/eye.png")).unwrap();
    treeview.append_column(&column);

    treeview.set_headers_visible(false);
    treeview.set_show_expanders(false);
    treeview.set_enable_search(false);
    treeview.set_activate_on_single_click(true);

    window.add(&box_container);
    window.show();
    box_container.show();
    search_entry.show();
    scrolled_window.realize();
    treeview.show();
    search_entry.grab_focus();

    window.set_opacity(0.9);

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

    let temp = selected_result.clone();
    treeview.connect_row_activated(move |treeview, path, column| {
        let selected = (*temp).borrow().clone();
        if let Some(search_result) = selected {
            execution::execute(search_result.exec);
            gtk::main_quit();
        }
    });

    search_entry.connect_activate(move |_| {
        let selected = selected_result.borrow().clone();
        if let Some(search_result) = selected {
            println!(" = {:?}", execution::execute(search_result.exec));

            gtk::main_quit();
        }
    });


    window.connect_key_release_event(move |s, key| {
        let keyval = key.get_keyval();
        let keystate = key.get_state();
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

    search_entry.connect_changed(move |s| {
        let text = s.get_buffer().get_text();
        completion_store.clear();
        if text.len() == 0 {
            if scrolled_window.is_visible() {
                scrolled_window.set_visible(false);
                window.resize(450, s.get_preferred_height().1);
            }
            return
        }

        scrolled_window.set_visible(true);

        let results = search_engine.search(&text).collect_vec();
        if results.len() > 0 {
            for result in results.iter() {
                //                let iter = completion_store.append();
                //                completion_store.set(&iter, &[0], &[&result.name]);

                if result.icon_path.is_none() {
                    completion_store.insert_with_values(None, &[0, 1],
                                                        &[&image, &result.name]);
                } else {
                    let image = Pixbuf::new_from_file_at_size((&result.icon_path).clone().unwrap().as_ref(), 64, 64).unwrap();
                    completion_store.insert_with_values(None, &[0, 1],
                                                        &[&image, &result.name]);
                }
            }

                * current_search_results.borrow_mut() = results;
            treeview.set_cursor(&TreePath::new_first(), None, false);

            window.resize(450, 400);
        }
    });

    gtk::main();
}

fn get_gtk_theme() -> Option<String> {
    utils::get_gsetting("org.gnome.desktop.interface", "gtk-theme")
}