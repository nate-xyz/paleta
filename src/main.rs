/* main.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod application;
mod config;
mod window;
mod database;
mod model;
mod pages;
mod dialog;
mod util;
mod toasts;
mod i18n;

use self::application::App;
use self::window::Window;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};

use std::{env, process};
use log::{debug, error};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    pretty_env_logger::init();

    // Set up gettext translations
    debug!("Setting up locale data");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    debug!("Loading resources");
    let resources = match env::var("MESON_DEVENV") {
        Err(_) => gio::Resource::load(PKGDATADIR.to_owned() + "/paleta.gresource")
            .expect("Unable to find paleta.gresource"),
        Ok(_) => match env::current_exe() {
            Ok(path) => {
                let mut resource_path = path;
                resource_path.pop();
                resource_path.push("paleta.gresource");
                gio::Resource::load(&resource_path)
                    .expect("Unable to find paleta.gresource in devenv")
            }
            Err(err) => {
                error!("Unable to find the current path: {}", err);
                process::exit(0x0100);
            }
        },
    };
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = App::new("io.github.nate_xyz.Paleta", &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}
