use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::util::{database, active_window};
use crate::toasts::{add_error_toast, add_success_toast};
use crate::i18n::{i18n, i18n_k};

use crate::model::palette::Palette;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/delete_palette_dialog.ui")]
    pub struct DeletePaletteDialogPriv {
        pub palette: RefCell<Option<Palette>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DeletePaletteDialogPriv {
        const NAME: &'static str = "DeletePaletteDialog";
        type Type = super::DeletePaletteDialog;
        type ParentType = adw::MessageDialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                palette: RefCell::new(None),
                name: RefCell::new(String::new()),
            }
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
        return delete_dialog;
    }

    fn initialize(&self) {
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "delete" {
                    this.delete_palette();
                }
            })
        );
    }

    fn load(&self, palette: &Palette) {
        self.set_heading(Some(&i18n_k("Delete {palette_name}?", &[("palette_name", &palette.name())])));
        self.imp().palette.replace(Some(palette.clone()));
    }

    fn delete_palette(&self) {
        let imp = self.imp();

        match imp.palette.borrow().as_ref() {
            Some(palette) => {
                if database().delete_palette(palette.id()) {
                    add_success_toast(&i18n("Removed"), &i18n_k("palette: «{palette_name}».", &[("palette_name", &palette.name())]));
                    return;
                } else {
                    add_error_toast(i18n_k("Unable to delete palette «{palette_name}».", &[("palette_name", &palette.name())]));
                }
            },
            None => add_error_toast(i18n("Unable to delete palette.")),
        }
    }
}
