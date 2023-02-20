use gtk::prelude::*;
use gtk::gio;

use std::path::Path;

use super::i18n::i18n_f;
use super::window::Window;

pub static SUCCESS_GREEN: &str = "\"#57e389\"";
pub static ERROR_RED: &str = "\"#c01c28\"";

// def add_toast_markup(self, title: str, timeout: int = 1):
// toast = Adw.Toast.new(title)
// toast.set_timeout(timeout)
// self.toast_overlay.add_toast(toast)

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

// def add_success_toast(self, verb: str, msg: str, timeout: int = 1):
// toast = Adw.Toast.new(f"<span foreground={SUCCESS_GREEN}>{verb}</span> {html.escape(msg)}")
// toast.set_timeout(timeout)
// self.toast_overlay.add_toast(toast)

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

// def add_error_toast(self, error: str, timeout: int = 1):
// # Translators: Only replace "Error!"
// toast = Adw.Toast.new(_(f"<span foreground={ERROR_RED}>Error!</span> {html.escape(error)}"))
// toast.set_timeout(timeout)
// self.toast_overlay.add_toast(toast)

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


// def open_image_toast(self, uri):
// base_name = html.escape(os.path.basename(uri))
// # Translators: Do not replace {base_name}, {SUCCESS_GREEN}, or the span tags, only translate "Opened image:"
// self.add_toast_markup(_(f"<span foreground={SUCCESS_GREEN}>Opened image:</span>  {base_name}"))

pub fn open_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    add_toast_markup(&i18n_f("<span foreground={}>Opened image:</span> {}", &[SUCCESS_GREEN, basename]));
}


// def error_image_toast(self, uri):
// base_name = os.path.basename(uri)
// # Translators: Do not replace {}
// self.add_error_toast(_("Could not open image: {}").format(base_name), 3)

pub fn error_image_toast(uri: &str) {
    let basename = Path::new(uri).file_name().unwrap().to_str().unwrap();
    add_error_toast(i18n_f("Could not open image: {}", &[basename]));
}


// def add_color_toast(self, hex_name, palette_name):
// # Translators: Do not replace {hex_name}, only translate "Added color" and "to palette"
// self.add_toast_markup(_(f"Added color <span foreground=\"{hex_name}\">{hex_name}</span> to palette «{html.escape(palette_name)}»."))

pub fn add_color_toast(hex_name: String, palette_name: String) {
    add_toast_markup(&i18n_f("Added color <span foreground=\"{}\">{}</span> to palette «{}».", &[&hex_name, &hex_name, &palette_name]));
}

// def remove_color_toast(self, hex_name, palette_name):
// # Translators: Do not replace {hex_name}, only translate "Removed color" and "from palette"
// self.add_toast_markup(_(f"Removed color <span foreground=\"{hex_name}\">{hex_name}</span> from palette «{html.escape(palette_name)}»."))

pub fn remove_color_toast(hex_name: String, palette_name: String) {
    add_toast_markup(&i18n_f("Removed color <span foreground=\"{}\">{}</span> from palette «{}».", &[&hex_name, &hex_name, &palette_name]));
}


