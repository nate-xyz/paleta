use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::util::{database, active_window};
use crate::toasts::{add_error_toast, add_success_toast};
use crate::i18n::{i18n, i18n_f};

use crate::model::palette::Palette;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/duplicate_palette_dialog.ui")]
    pub struct DuplicatePaletteDialogPriv {
        #[template_child(id = "adw_entry_row")]
        pub adw_entry_row: TemplateChild<adw::EntryRow>,

        pub palette: RefCell<Option<Palette>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DuplicatePaletteDialogPriv {
        const NAME: &'static str = "DuplicatePaletteDialog";
        type Type = super::DuplicatePaletteDialog;
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

    impl ObjectImpl for DuplicatePaletteDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for DuplicatePaletteDialogPriv {}
    impl WindowImpl for DuplicatePaletteDialogPriv {}
    impl MessageDialogImpl for DuplicatePaletteDialogPriv {}
    impl DuplicatePaletteDialogPriv {}
}

glib::wrapper! {
    pub struct DuplicatePaletteDialog(ObjectSubclass<imp::DuplicatePaletteDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl DuplicatePaletteDialog {
    pub fn new(palette: &Palette) -> DuplicatePaletteDialog {
        let duplicate_dialog: DuplicatePaletteDialog = glib::Object::builder::<DuplicatePaletteDialog>().build();
        duplicate_dialog.load(palette);
        duplicate_dialog
    }

    fn initialize(&self) {
        self.set_transient_for(Some(&active_window().unwrap()));
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "duplicate" {
                    this.duplicate_palette();
                }
            }),
        );
    }

    fn load(&self, palette: &Palette) {
        self.set_heading(Some(&i18n_f("Duplicate {}?", &[&palette.name()])));
        self.set_name(i18n_f("{} duplicate", &[&palette.name()]));
        self.imp().palette.replace(Some(palette.clone()));
    }

    fn set_name(&self, name: String) {
        self.imp().adw_entry_row.set_text(name.as_str());
        self.imp().name.replace(name);
    }

    fn duplicate_palette(&self) {
        let imp = self.imp();
        match imp.palette.borrow().as_ref() {
            Some(palette) => {
                let mut name = imp.adw_entry_row.text().to_string();
                if name == "" {
                    name = imp.name.borrow().clone();
                }
    
                if database().duplicate_palette(palette.id(), name.clone()) {
                    add_success_toast(&i18n("Duplicated!"), &i18n_f("Copied «{}» to «{}».", &[&palette.name(), &name]));
                    return;
                } else {
                    add_error_toast(i18n_f("Unable to duplicate palette «{}».", &[&name]));
                }
            },
            None => add_error_toast(i18n("Unable to duplicate palette.")),
        }
    }

}
