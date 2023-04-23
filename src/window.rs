/* window.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::{prelude::*, gio, glib, glib::{clone, Sender}, gdk, CompositeTemplate};
use gtk_macros::send;

use std::{cell::RefCell, error::Error};
use log::{debug, error};

use crate::pages::{
    image_drop::image_drop_page::ImageDropPage,
    palette::palette_page::PalettePage,
};
use crate::database::DatabaseAction;
use crate::toasts::add_error_toast;
use crate::i18n::i18n;

use super::util::{database, model, settings_manager};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/window.ui")]
    pub struct Window {

        #[template_child(id = "toast_overlay")]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        
        #[template_child(id = "header_bar")]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        
        #[template_child(id = "view-switcher-title")]
        pub view_switcher_title: TemplateChild<adw::ViewSwitcherTitle>,

        #[template_child(id = "open_image_button")]
        pub open_image_button: TemplateChild<gtk::Button>,

        #[template_child(id = "stack")]
        pub stack: TemplateChild<adw::ViewStack>,

        #[template_child(id = "image_drop_page")]
        pub image_drop_page: TemplateChild<ImageDropPage>,

        #[template_child(id = "palette_page")]
        pub palette_page: TemplateChild<PalettePage>,

        #[template_child(id = "edit_palette_button")]
        pub edit_palette_button: TemplateChild<gtk::Button>,

        pub clipboard: Option<gdk::Clipboard>,
        pub open_image_dialog: RefCell<Option<gtk::FileChooserNative>>,
        pub settings: gio::Settings,
        pub db_sender: Sender<DatabaseAction>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                toast_overlay: TemplateChild::default(),
                header_bar: TemplateChild::default(),
                view_switcher_title: TemplateChild::default(),
                open_image_button: TemplateChild::default(),
                stack: TemplateChild::default(),
                image_drop_page: TemplateChild::default(),
                palette_page: TemplateChild::default(),
                edit_palette_button: TemplateChild::default(),
                clipboard: Some(gdk::Display::default().unwrap().clipboard()),
                open_image_dialog: RefCell::new(None),
                settings: settings_manager(),
                db_sender: database().sender(),
            }
        }

    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: Window = glib::Object::builder()
            .property("application", application)
            .build();
        window.setup();
        window
    }


    fn setup(&self) {
        let imp = self.imp();

        self.setup_settings();
        self.bind_signals();
        self.add_dialog();

        send!(imp.db_sender, DatabaseAction::TryLoadingDataBase);
    }

    // fn add_help_overlay(&self) {
    //     let help_overlay = gtk::Builder::from_resource("/io/github/nate_xyz/Paleta/help-overlay.ui").object::<gtk::ShortcutsWindow>("help_overlay").unwrap();
    //     self.set_help_overlay(Some(&help_overlay));
    // }

    fn bind_signals(&self) {
        debug!("bind signals");
        let imp = self.imp();

        // self.edit_palette_button.connect('clicked', lambda _button: self.palette_page.toggle_edit_mode())
        imp.edit_palette_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.imp().palette_page.toggle_edit_mode();
            }),
        );

        // self.stack.connect('notify::visible-child-name', self.on_stack_switch)
        imp.stack.connect_notify_local(
            Some("visible-child-name"),
            clone!(@weak self as this => move |stack, _| {
                if stack.visible_child_name().unwrap().as_str() == "palette-stack-page" && model().palettes().len() != 0 {
                    this.imp().edit_palette_button.show();
                } else {
                    this.imp().edit_palette_button.hide();
                }
            }),
        );

        self.connect_local(
            "unrealize",
            false,
            clone!(@strong self as this => @default-return None, move |_value| {
                this.save_window_props();
                None
            }),
        );
    }

    fn go_to_image_drop_page(&self) {
        if self.imp().stack.visible_child_name().unwrap().as_str() != "drop-stack-page" {
            self.imp().stack.set_visible_child(&*self.imp().image_drop_page)
        }
    }

    pub fn go_to_palette_page(&self) {
        if self.imp().stack.visible_child_name().unwrap().as_str() != "palette-stack-page" {
            self.imp().stack.set_visible_child(&*self.imp().palette_page)
        }
    }

    pub fn add_toast(&self, toast: adw::Toast) {
        self.imp().toast_overlay.add_toast(toast);
    }

    fn add_dialog(&self) {
        let dialog = gtk::FileChooserNative::builder()
            .accept_label(&i18n("_Open Image"))
            .cancel_label(&i18n("_Cancel"))
            .modal(true)
            .title(&i18n("Select an Image File"))
            .action(gtk::FileChooserAction::Open)
            .select_multiple(false)
            .transient_for(self)
            .build();

        //let filter = gtk::FileFilter::new();
        // gtk::FileFilter::set_name(&filter, Some(&i18n("Image files")));
        // filter.add_mime_type("image/*");
        // dialog.add_filter(&filter);
        dialog.connect_response(
            clone!(@weak self as this => move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    match this.load_image(dialog.file()) {
                        Ok(uri) => {
                            if this.imp().image_drop_page.load_image(uri.as_str()) {
                                this.go_to_image_drop_page();
                            } else {
                                add_error_toast(i18n("Unable to load file."));
                            }
                        },
                        Err(e) => {
                            add_error_toast(i18n("Unable to load file."));
                            error!("{}", e);
                        },
                    }
                }
            }),
        );

        self.imp().open_image_dialog.replace(Some(dialog));

        self.imp().open_image_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.imp().open_image_dialog.borrow().as_ref().unwrap().show();
            }),
        );

        self.imp().image_drop_page.imp().open_image_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.imp().open_image_dialog.borrow().as_ref().unwrap().show();
            }),
        );

        
    }

    fn load_image(&self, file_option: Option<gio::File>) -> Result<String, Box<dyn Error>> {
        let file = file_option.ok_or("No file.")?;
        let path = file.path().ok_or("No path.")?;
        let uri = path.to_str().ok_or("No uri.")?.to_owned();
        Ok(uri)
    }

    pub fn edit_button_set_visible(&self, visible: bool) {
        let button: &gtk::Button = self.imp().edit_palette_button.as_ref();
        button.set_visible(visible);
    }

    pub fn edit_button_mode(&self, mode: bool) {
        let button: &gtk::Button = self.imp().edit_palette_button.as_ref();
        if mode {
            button.set_css_classes(&[&"opaque", &"edit-action-button"]);
        } else {
            button.set_css_classes(&[&"flat"]);
        }
    }

    pub fn copy_color(&self, hex_name: String) {
        self.imp().clipboard.as_ref().unwrap().set_text(hex_name.as_str());
    }

    /*
    GIO SETTINGS
    */

    fn setup_settings(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let maximized = imp.settings.boolean("window-maximized");

        self.set_default_size(width, height);
        self.set_maximized(maximized);
    }

    fn save_window_props(&self) {
        let imp = self.imp();

        let (width, height) = self.default_size();
        let maximized = self.is_maximized();

        _ = imp.settings.set_int("window-width", width);
        _ = imp.settings.set_int("window-height", height);
        _ = imp.settings.set_boolean("window-maximized", maximized);
    }
}
