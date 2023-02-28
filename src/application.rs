/* application.rs
 *
 * Copyright 2023 nate-xyz
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, glib::clone};
use gtk_macros::action;

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
    pub fn new() -> Self {
        glib::Object::builder::<App>()
            .property("application-id", &"io.github.nate_xyz.Paleta")
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            .property("resource-base-path", &"/io/github/nate_xyz/Paleta")
            .build()
    }

    pub fn model(&self) -> Rc<Model> {
        self.imp().model.clone()
    }

    pub fn database(&self) -> Rc<Database> {
        self.imp().database.clone()
    }

    fn setup_gactions(&self) {
        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                app.quit()
            })
        );

        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about()
            })
        );
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
            .copyright("© 2023 nate-xyz")
            .license_type(gtk::License::Gpl30Only)
            .website("https://github.com/nate-xyz/paleta")
            .issue_url("https://github.com/nate-xyz/paleta/issues")
            .build();
        
        // Translator credits. Replace "translator-credits" with your name/username, and optionally an email or URL. 
        // One name per line, please do not remove previous names.
        about.set_translator_credits(&i18n("translator-credits"));

        // Translators: only replace "Powered by "
        let ack: String = i18n("Powered by color-thief");

        about.add_acknowledgement_section(Some(&ack), 
            &["color-thief-rs https://github.com/RazrFalcon/color-thief-rs", "color-thief-py https://github.com/fengsp/color-thief-py", "color-thief https://github.com/lokesh/color-thief"]);

        about.present();
    }
}
