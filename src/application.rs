/* application.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, glib::clone};

use std::rc::Rc;

use crate::config::VERSION;
use crate::Window;
use crate::database::Database;
use crate::model::model::Model;
use crate::i18n::i18n;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct App {
        pub database: Rc<Database>,
        pub model: Rc<Model>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type Type = super::App;
        type ParentType = adw::Application;

        fn new() -> Self {
            let database = Rc::new(Database::new());
            let model = Model::new();
            database.connect_local(
                "populate-model",
                false,
                clone!(@weak model => @default-return None, move |_args| {
                    model.populate_all();
                    None
                }),
            );
            model.load_db(database.clone());
            Self {
                database,
                model: Rc::new(model),
            }
        }
    }

    impl ObjectImpl for App {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_gactions();

            obj.set_accels_for_action("app.quit", &["<primary>q", "<primary>w"]);
            // TODO: Add more accelerators
        }
    }

    impl ApplicationImpl for App {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = Window::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for App {}
    impl AdwApplicationImpl for App {}
}

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    pub fn model(&self) -> Rc<Model> {
        self.imp().model.clone()
    }
    
    pub fn database(&self) -> Rc<Database> {
        self.imp().database.clone()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Paleta")
            .application_icon("io.github.nate_xyz.Paleta")
            .developer_name("nate-xyz")
            .version(VERSION)
            .developers(vec!["nate-xyz"])
            .copyright("Copyright © 2023 nate-xyz")
            .license_type(gtk::License::Gpl30Only)
            // Translator credits. Replace "translator-credits" with your name/username, and optionally an email or URL.
            // One name per line, please do not remove previous names.
            .translator_credits(&i18n("translator-credits"))
            .website("https://github.com/nate-xyz/paleta")
            .issue_url("https://github.com/nate-xyz/paleta/issues")
            .build();

        // Translators: only replace "Powered by "
        let ack: String = i18n("Powered by color-thief");

        about.add_acknowledgement_section(Some(&ack),
            &["color-thief-rs https://github.com/RazrFalcon/color-thief-rs", "color-thief https://github.com/lokesh/color-thief"]);

        about.present();
    }
}
