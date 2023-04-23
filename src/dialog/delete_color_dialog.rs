/* delete_color_dialog.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{glib, glib::{clone, Sender}, CompositeTemplate};
use gtk_macros::send;

use std::cell::RefCell;
use log::error;

use crate::model::{
    palette::Palette,
    color::Color,
};
use crate::database::DatabaseAction;
use crate::toasts::add_error_toast;
use crate::i18n::{i18n, i18n_k};
use crate::util::{database, active_window};

use super::simpler_delete_color_card::SimplerDeleteColorCard;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/delete_color_dialog.ui")]
    pub struct DeleteColorDialogPriv {
        #[template_child(id = "color_bin")]
        pub color_bin: TemplateChild<adw::Bin>,

        pub db_sender: Sender<DatabaseAction>,
        pub palette: RefCell<Option<Palette>>,
        pub color: RefCell<Option<Color>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DeleteColorDialogPriv {
        const NAME: &'static str = "DeleteColorDialog";
        type Type = super::DeleteColorDialog;
        type ParentType = adw::MessageDialog;

        fn new() -> Self {
            Self {
                color_bin: TemplateChild::default(),
                db_sender: database().sender(),
                palette: RefCell::new(None),
                color: RefCell::new(None),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DeleteColorDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for DeleteColorDialogPriv {}
    impl WindowImpl for DeleteColorDialogPriv {}
    impl MessageDialogImpl for DeleteColorDialogPriv {}
    impl DeleteColorDialogPriv {}
}

glib::wrapper! {
    pub struct DeleteColorDialog(ObjectSubclass<imp::DeleteColorDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl DeleteColorDialog {
    pub fn new(color: &Color, palette: &Palette) -> DeleteColorDialog {
        let save_dialog: DeleteColorDialog = glib::Object::builder::<DeleteColorDialog>().build();
        save_dialog.load(color, palette);
        save_dialog
    }

    fn initialize(&self) {        
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "remove" {
                    this.remove_color_from_palette();
                }
            }),
        );
       
    }

    fn load(&self, color: &Color, palette: &Palette) {
        self.set_heading(Some(&i18n_k("Remove color {color_hex} from {palette_name}?", &[("color_hex", &color.hex_name()), ("palette_name", &palette.name())])));
        let imp = self.imp();
        imp.color_bin.set_child(Some(&SimplerDeleteColorCard::new(color)));        
        imp.color.replace(Some(color.clone()));
        imp.palette.replace(Some(palette.clone()));
    }

    fn remove_color_from_palette(&self) {
        let imp = self.imp();
    
        if let Some(palette) = imp.palette.borrow().as_ref() {
            if let Some(color) = imp.color.borrow().as_ref() {
                send!(imp.db_sender, DatabaseAction::RemoveColorFromPalette((color.id(), color.hex_name(), palette.id(), palette.name())));
                return;
            }
            add_error_toast(i18n("Unable to remove color."));
        }
    }
}
