use gtk::{prelude::*, gio};

use std::path::Path;

use super::window::Window;
use super::i18n::i18n_k;

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

        // Translators: Only replace "Error!". Reorder if necessary
        let toast = adw::Toast::new(&i18n_k("<span foreground={ERROR_RED}>Error!</span> {error_msg}", &[("ERROR_RED", ERROR_RED), ("error_msg", &html_escape::encode_text_minimal(&msg))]));

        toast.set_timeout(1);
        win.add_toast(toast);
}

pub fn open_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    // Translators: Only replace "Opened image:". Reorder if necessary
    add_toast_markup(&i18n_k("<span foreground={SUCCESS_GREEN}>Opened image:</span> {image_name}", &[("SUCCESS_GREEN", SUCCESS_GREEN), ("image_name", basename)]));
}

pub fn error_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    // Translators: Do not replace {image_name}
    add_error_toast(i18n_k("Could not open image: {image_name}", &[("image_name", basename)]));
}

pub fn add_color_toast(hex_name: String, palette_name: String) {
    let color_hex = format!("<span foreground=\"{}\">{}</span>", hex_name, hex_name);
    // Translators: Do not replace {color_hex} or {palette_name}, only translate "Added color" and "to palette" Reorder if necessary
    add_toast_markup(&i18n_k("Added color {color_hex} to palette «{palette_name}».", &[("color_hex", &color_hex), ("palette_name", &palette_name)]));
}


pub fn remove_color_toast(hex_name: String, palette_name: String) {
    let color_hex = format!("<span foreground=\"{}\">{}</span>", hex_name, hex_name);
    // Translators: Do not replace {color_hex} or {palette_name}, only translate "Removed color" and "from palette" Reorder if necessary
    add_toast_markup(&i18n_k("Removed color {color_hex} from palette «{palette_name}».", &[("color_hex", &color_hex), ("palette_name", &palette_name)]));
}


