use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::util::{ database, active_window};
use crate::toasts::{add_error_toast, add_success_toast};
use crate::i18n::{i18n, i18n_k};

use crate::model::palette::Palette;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/rename_palette_dialog.ui")]
    pub struct RenamePaletteDialogPriv {
        #[template_child(id = "adw_entry_row")]
        pub adw_entry_row: TemplateChild<adw::EntryRow>,

        pub palette: RefCell<Option<Palette>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RenamePaletteDialogPriv {
        const NAME: &'static str = "RenamePaletteDialog";
        type Type = super::RenamePaletteDialog;
        type ParentType = adw::MessageDialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                adw_entry_row: TemplateChild::default(),
                palette: RefCell::new(None),
                name: RefCell::new(String::new()),
            }
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
        return save_dialog;
    }

    fn initialize(&self) {
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "rename" {
                    this.rename_palette();
                }
            })
        );
    }

    fn load(&self, palette: &Palette) {
        self.set_heading(Some(&i18n_k("Rename {palette_name}?", &[("palette_name", &palette.name())])));
        self.set_name(palette.name());
        self.imp().palette.replace(Some(palette.clone()));
    }

    fn set_name(&self, name: String) {
        self.imp().adw_entry_row.set_text(name.as_str());
        self.imp().name.replace(name);
    }

    fn rename_palette(&self) {
        let imp = self.imp();

        match imp.palette.borrow().as_ref() {
            Some(palette) => {
                let mut name = imp.adw_entry_row.text().to_string();

                if name == "" {
                    name = imp.name.borrow().clone();
                }

                if database().rename_palette(palette.id(), name.clone()) {
                    add_success_toast(&i18n("Renamed!"), &i18n_k("Changed name from «{old_palette_name}» to «{new_palette_name}».", &[("old_palette_name", &palette.name()), ("new_palette_name", &name)]));
                    return;
                } else {
                    add_error_toast(i18n_k("Unable to rename palette «{palette_name}».", &[("palette_name", &name)]));
                }
            },
            None => add_error_toast(i18n("Unable to rename palette.")),
        }
    }
}
