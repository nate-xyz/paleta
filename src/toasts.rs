use gtk::prelude::*;
use gtk::gio;

use std::path::Path;

use super::i18n::i18n_f;
use super::window::Window;

pub static SUCCESS_GREEN: &str = "\"#57e389\"";
pub static ERROR_RED: &str = "\"#c01c28\"";

pub fn add_toast_markup(msg: &str) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

        let toast = adw::Toast::new(msg);
        toast.set_timeout(1);
        win.add_toast(toast);
}

pub fn add_success_toast(verb: &str, msg: &str) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();

        let toast = adw::Toast::new(format!("<span foreground={}>{}</span> {}", SUCCESS_GREEN, verb, html_escape::encode_text_minimal(&msg)).as_str());
        toast.set_timeout(1);
        win.add_toast(toast);
}

pub fn add_error_toast(msg: String) {
    let app = gio::Application::default()
        .expect("Failed to retrieve application singleton")
        .downcast::<gtk::Application>()
        .unwrap();
    
    let win = app
        .active_window()
        .unwrap()
        .downcast::<Window>()
        .unwrap();
    
        let toast = adw::Toast::new(&i18n_f("<span foreground={}>Error!</span> {}", &[ERROR_RED, &html_escape::encode_text_minimal(&msg)]));

        toast.set_timeout(1);
        win.add_toast(toast);
}

pub fn open_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    add_toast_markup(&i18n_f("<span foreground={}>Opened image:</span> {}", &[SUCCESS_GREEN, basename]));
}

pub fn error_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    add_error_toast(i18n_f("Could not open image: {}", &[basename]));
}

pub fn add_color_toast(hex_name: String, palette_name: String) {
    add_toast_markup(&i18n_f("Added color <span foreground=\"{}\">{}</span> to palette «{}».", &[&hex_name, &hex_name, &palette_name]));
}


pub fn remove_color_toast(hex_name: String, palette_name: String) {
    add_toast_markup(&i18n_f("Removed color <span foreground=\"{}\">{}</span> from palette «{}».", &[&hex_name, &hex_name, &palette_name]));
}


