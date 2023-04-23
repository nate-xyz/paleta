/* extracted_color.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::{glib, subclass::prelude::*};

use std::{cell::Cell, cell::RefCell};

use crate::util::rgb_to_hex;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ExtractedColorPriv {
        pub rgb_tuple: Cell<(u8, u8, u8)>,
        pub rgba_tuple: Cell<(u8, u8, u8, f32)>,
        pub hex_name: RefCell<String>,
        pub rgb_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExtractedColorPriv {
        const NAME: &'static str = "ExtractedColor";
        type Type = super::ExtractedColor;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ExtractedColorPriv {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct ExtractedColor(ObjectSubclass<imp::ExtractedColorPriv>);
}

impl ExtractedColor {
    pub fn new(rgba: (u8, u8, u8)) -> ExtractedColor {
        let ec: ExtractedColor = glib::Object::builder::<ExtractedColor>().build();
        ec.load(rgba);
        ec
    }

    fn load(&self, rgba: (u8, u8, u8)) {
        let imp = self.imp();

        let red = rgba.0;
        let green = rgba.1;
        let blue = rgba.2;

        imp.rgb_tuple.set(rgba);
        imp.rgba_tuple.set((red, green, blue, 1.0));
        imp.hex_name.replace(format!("{}", rgb_to_hex(red, green, blue)));
        imp.rgb_name.replace(format!("rgb({},{},{})", red, green, blue));
    }

    pub fn rgb_tuple(&self) -> (u8, u8, u8) {
        self.imp().rgb_tuple.get()
    }

    pub fn rgba_tuple(&self) -> (u8, u8, u8, f32) {
        self.imp().rgba_tuple.get()
    }

    pub fn hex_name(&self) -> String {
        self.imp().hex_name.borrow().clone()
    }

    pub fn rgb_name(&self) -> String {
        self.imp().rgb_name.borrow().clone()
    }
}
    