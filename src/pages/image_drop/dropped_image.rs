/* dropped_image.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{glib, gdk_pixbuf};

use std::{cell::RefCell, path::Path, error::Error};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DroppedImagePriv {
        pub picture: RefCell<gtk::Picture>,
        pub pixbuf: RefCell<Option<gdk_pixbuf::Pixbuf>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DroppedImagePriv {
        const NAME: &'static str = "DroppedImage";
        type Type = super::DroppedImage;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for DroppedImagePriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for DroppedImagePriv {}
    impl BinImpl for DroppedImagePriv {}

    impl DroppedImagePriv {}
}

glib::wrapper! {
    pub struct DroppedImage(ObjectSubclass<imp::DroppedImagePriv>)
    @extends gtk::Widget, adw::Bin;
}


impl DroppedImage {
    pub fn new() -> DroppedImage {
        let dropped_image: DroppedImage = glib::Object::builder::<DroppedImage>().build();
        dropped_image
    }

    fn initialize(&self) {
      self.set_halign(gtk::Align::Center);
      self.set_valign(gtk::Align::Center);
    }

    pub fn load_image(&self, image_path: &str) -> Result<(), Box<dyn Error>> {
        let imp = self.imp();
        let path = Path::new(image_path);
        let basename = path.file_name().unwrap();
        let _image_name = format!("{}", html_escape::encode_text_minimal(basename.to_str().unwrap()));
        let pixbuf = gdk_pixbuf::Pixbuf::from_file(image_path)?;
        let picture = gtk::Picture::for_pixbuf(&pixbuf);
        self.set_child(Some(&picture));
        imp.picture.replace(picture);
        imp.pixbuf.replace(Some(pixbuf));
        Ok(())
    }
}
    