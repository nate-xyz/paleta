/* rename_palette_dialog.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{glib, glib::{clone, Sender}, CompositeTemplate};
use gtk_macros::send;

use std::cell::RefCell;
use log::error;

use crate::model::palette::Palette;
use crate::database::DatabaseAction;
use crate::toasts::add_error_toast;
use crate::util::{database, active_window};
use crate::i18n::{i18n, i18n_k};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/rename_palette_dialog.ui")]
    pub struct RenamePaletteDialogPriv {
        #[template_child(id = "adw_entry_row")]
        pub adw_entry_row: TemplateChild<adw::EntryRow>,

        pub db_sender: Sender<DatabaseAction>,
        pub palette: RefCell<Option<Palette>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RenamePaletteDialogPriv {
        const NAME: &'static str = "RenamePaletteDialog";
        type Type = super::RenamePaletteDialog;
        type ParentType = adw::MessageDialog;

        fn new() -> Self {
            Self {
                adw_entry_row: TemplateChild::default(),
                db_sender: database().sender(),
                palette: RefCell::new(None),
                name: RefCell::new(String::new()),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RenamePaletteDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for RenamePaletteDialogPriv {}
    impl WindowImpl for RenamePaletteDialogPriv {}
    impl MessageDialogImpl for RenamePaletteDialogPriv {}
    impl RenamePaletteDialogPriv {}
}

glib::wrapper! {
    pub struct RenamePaletteDialog(ObjectSubclass<imp::RenamePaletteDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl RenamePaletteDialog {
    pub fn new(palette: &Palette) -> RenamePaletteDialog {
        let save_dialog: RenamePaletteDialog = glib::Object::builder::<RenamePaletteDialog>().build();
        save_dialog.load(palette);
        save_dialog
    }

    fn initialize(&self) {
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "rename" {
                    this.rename_palette();
                }
            }),
        );
    }

    fn load(&self, palette: &Palette) {
        self.set_heading(Some(&i18n_k("Rename {palette_name}?", &[("palette_name", &palette.name())])));
        self.set_name(palette.name());
        self.imp().palette.replace(Some(palette.clone()));
    }

    fn set_name(&self, name: String) {
        let imp = self.imp();
        imp.adw_entry_row.set_text(name.as_str());
        imp.name.replace(name);
    }

    fn rename_palette(&self) {
        let imp = self.imp();

        if let Some(palette) = imp.palette.borrow().as_ref() {
            let mut name = imp.adw_entry_row.text().to_string();
            if name == "" {
                name = imp.name.borrow().clone();
            }

            send!(imp.db_sender, DatabaseAction::RenamePalette((palette.id(), palette.name(), name)));
            return;
        }
        add_error_toast(i18n("Unable to rename palette."));
    }

}
