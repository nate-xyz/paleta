/* delete_palette_dialog.rs
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
use crate::i18n::{i18n, i18n_k};
use crate::util::{database, active_window};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/delete_palette_dialog.ui")]
    pub struct DeletePaletteDialogPriv {
        pub db_sender: Sender<DatabaseAction>,
        pub palette: RefCell<Option<Palette>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DeletePaletteDialogPriv {
        const NAME: &'static str = "DeletePaletteDialog";
        type Type = super::DeletePaletteDialog;
        type ParentType = adw::MessageDialog;

        fn new() -> Self {
            Self {
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

    impl ObjectImpl for DeletePaletteDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for DeletePaletteDialogPriv {}
    impl WindowImpl for DeletePaletteDialogPriv {}
    impl MessageDialogImpl for DeletePaletteDialogPriv {}
    impl DeletePaletteDialogPriv {}
}

glib::wrapper! {
    pub struct DeletePaletteDialog(ObjectSubclass<imp::DeletePaletteDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl DeletePaletteDialog {
    pub fn new(palette: &Palette) -> DeletePaletteDialog {
        let delete_dialog: DeletePaletteDialog = glib::Object::builder::<DeletePaletteDialog>().build();
        delete_dialog.load(palette);
        delete_dialog
    }

    fn initialize(&self) {
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "delete" {
                    this.delete_palette();
                }
            }),
        );
    }

    fn load(&self, palette: &Palette) {
        self.set_heading(Some(&i18n_k("Delete {palette_name}?", &[("palette_name", &palette.name())])));
        self.imp().palette.replace(Some(palette.clone()));
    }

    fn delete_palette(&self) {
        let imp = self.imp();

        if let Some(palette) = imp.palette.borrow().as_ref() {
            send!(imp.db_sender, DatabaseAction::DeletePalette((palette.id(), palette.name())));
            return;
        }
        add_error_toast(i18n("Unable to delete palette."));
    }

}
