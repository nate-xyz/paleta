use gtk::prelude::*;
use gtk::gio;

use std::rc::Rc;

use crate::database::Database;
use crate::model::model::Model;

use super::application::App;
use super::window::Window;

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn model() -> Rc<Model>{
    gio::Application::default()
    .expect("Failed to retrieve application singleton")
    .downcast::<App>()
    .unwrap()
    .model()
}

pub fn database() -> Rc<Database>{
    gio::Application::default()
    .expect("Failed to retrieve application singleton")
    .downcast::<App>()
    .unwrap()
    .database()
}

pub fn active_window() -> Option<gtk::Window> {
    let app = gio::Application::default()
    .expect("Failed to retrieve application singleton")
    .downcast::<gtk::Application>()
    .unwrap();

    let win = app
    .active_window();

    win
}

pub fn edit_button_set_visible(visible: bool) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

    win.edit_button_set_visible(visible);
    win.go_to_palette_page();
}

pub fn go_to_palette_page() {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

    win.go_to_palette_page();
}


pub fn edit_button_mode(mode: bool) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

    win.edit_button_mode(mode);
}

pub fn copy_color(hex_name: String) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

    win.copy_color(hex_name.clone());

    let msg = format!("Copied color <span foreground=\"{}\">{}</span> to clipboard!", hex_name, hex_name);
    let toast = adw::Toast::new(msg.as_str());
    toast.set_timeout(1);
    win.add_toast(toast);
}



