use gtk::glib;
use gtk::subclass::prelude::*;

use std::{cell::Cell, cell::RefCell};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ColorPriv {
        pub id: Cell<i64>,
        pub red: Cell<i64>,
        pub green: Cell<i64>,
        pub blue: Cell<i64>,
        pub alpha: Cell<f64>,
        pub rgb: Cell<(i64, i64, i64)>,
        pub rgba: Cell<(i64, i64, i64, f64)>,
        pub hex_name: RefCell<String>,
        pub rgb_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorPriv {
        const NAME: &'static str = "Color";
        type Type = super::Color;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ColorPriv {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct Color(ObjectSubclass<imp::ColorPriv>);
}

impl Color {
    pub fn new(id: i64, r: i64, g: i64, b: i64, a: f64, hex: String) -> Color {
        let color: Color = glib::Object::builder::<Color>().build();
        color.load(id, r, g, b, a, hex);
        color
    }

    fn load(&self, id: i64, r: i64, g: i64, b: i64, a: f64, hex: String) {
        let imp = self.imp();
        imp.id.set(id);
        imp.red.set(r);
        imp.green.set(g);
        imp.blue.set(b);
        imp.alpha.set(a);
        imp.rgb.set((r, g, b));
        imp.rgba.set((r, g, b, a));
        imp.hex_name.replace(hex);
        imp.rgb_name.replace(format!("rgb({},{},{})", r, g, b));
    }

    pub fn id(&self) -> i64 {
        self.imp().id.get()
    }
    
    pub fn rgb(&self) -> (i64, i64, i64) {
        self.imp().rgb.get()
    }

    pub fn rgba(&self) -> (i64, i64, i64, f64) {
        self.imp().rgba.get()
    }

    pub fn hex_name(&self) -> String {
        self.imp().hex_name.borrow().clone()
    }

    pub fn rgb_name(&self) -> String {
        self.imp().rgb_name.borrow().clone()
    }

}
    