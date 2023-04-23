/* image_drop_page.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk::ContentFormats, gdk, gio, glib, glib::clone, CompositeTemplate};

use std::{cell::Cell, path::Path};
use log::{debug, error};

use crate::toasts::{add_error_toast, open_image_toast, error_image_toast};
use crate::i18n::i18n;

use super::color_thief_panel::ColorThiefPanel;
use super::dropped_image::DroppedImage;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/image_drop_page.ui")]
    pub struct ImageDropPagePriv {
        #[template_child(id = "overlay")]
        pub overlay: TemplateChild<gtk::Overlay>,

        #[template_child(id = "status")]
        pub status: TemplateChild<adw::StatusPage>,

        #[template_child(id = "image_bin")]
        pub image_bin: TemplateChild<adw::Bin>,

        #[template_child(id = "thief_panel")]
        pub thief_panel: TemplateChild<ColorThiefPanel>,

        #[template_child(id = "open_image_button")]
        pub open_image_button: TemplateChild<gtk::Button>,

        pub file_verified: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageDropPagePriv {
        const NAME: &'static str = "ImageDropPage";
        type Type = super::ImageDropPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImageDropPagePriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for ImageDropPagePriv {}
    impl BinImpl for ImageDropPagePriv {}

    impl ImageDropPagePriv {}
}

glib::wrapper! {
    pub struct ImageDropPage(ObjectSubclass<imp::ImageDropPagePriv>)
    @extends gtk::Widget, adw::Bin;
}

impl ImageDropPage {
    pub fn new() -> ImageDropPage {
        let color_panel: ImageDropPage = glib::Object::builder::<ImageDropPage>().build();
        color_panel
    }

    fn initialize(&self) {
        self.imp().file_verified.set(false);
        self.setup_drop_target();
    }

    fn setup_drop_target(&self) {
        let imp = self.imp();

        let formats = gdk::ContentFormats::new(&["text/uri-list"]);
        let drop_target = gtk::DropTargetAsync::new(Some(formats), gdk::DragAction::COPY);

        drop_target.connect_accept(
            clone!(@strong self as this => move |_drop_target, drop_value| {
                this.imp().file_verified.set(false);
                let formats = drop_value.formats();
                if contain_mime_type(formats) {
                    drop_value.read_value_async(gio::File::static_type(), glib::PRIORITY_DEFAULT, None::<&gio::Cancellable>, 
                    clone!(@strong this => move |value| {
                            this.verify_file_valid(value)
                        })
                    );
                    return true
                }
                return false
            })
        );

        drop_target.connect_drop(
            clone!(@strong self as this => move |_drop_target, drop_value, _x, _y| {
                drop_value.read_value_async(gio::File::static_type(), glib::PRIORITY_DEFAULT, None::<&gio::Cancellable>, 
                    clone!(@strong this => move |value| {
                        if !this.imp().file_verified.get() {
                            add_error_toast(i18n("Unable to verify file on drop, try with the file chooser in the upper left corner."))
                        }
                        this.load_value_async(value)
                    })
                );
                return true
            }),
        );

        imp.overlay.add_controller(drop_target);
    }

    fn verify_file_valid(&self, value: Result<glib::Value, glib::Error>) {
        match value {
            Ok(value) => {
                let file: gio::File = value.get::<gio::File>().unwrap();
                let uri = file.path().unwrap();
                self.imp().file_verified.set(Path::new(&uri).exists());
            },
            Err(e) => {
                error!("{}", e);
                return;
            }
        }

    }

    fn load_value_async(&self, value: Result<glib::Value, glib::Error>) {
        match value {
            Ok(value) => {
                let file: gio::File = value.get::<gio::File>().unwrap();
                let uri = file.path().unwrap();
                self.load_image(uri.to_str().unwrap());
                debug!("{:?}", uri);
            },
            Err(e) => {
                add_error_toast(i18n("Unable to read drop."));
                error!("{}", e);
                return;
            }
        }
    }

    pub fn load_image(&self, uri: &str) -> bool {
        let dropped_image = DroppedImage::new();
        if let Err(e) = dropped_image.load_image(uri) {
            error_image_toast(uri);
            error!("{}", e);
            return false
        } else {
            let imp = self.imp();
            imp.thief_panel.set_image(dropped_image);
            imp.status.hide();
            open_image_toast(uri);
            return true;
        }
    }


    fn set_image(&self, image: DroppedImage) {
        let imp = self.imp();
        self.imp().thief_panel.set_visible(true);
        imp.image_bin.set_child(Some(&image));
        self.imp().thief_panel.imp().image.replace(Some(image));
        self.imp().thief_panel.list_store().remove_all();
        self.imp().thief_panel.start_extraction();
    }

}

fn contain_mime_type(formats: ContentFormats) -> bool {
    let mimes = ["text/uri-list"];
    mimes.iter().any(|m| formats.contain_mime_type(m))
}