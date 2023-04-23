use adw::{prelude::*, subclass::prelude::*};
use gtk::{glib, glib::clone, CompositeTemplate, gio};

use std::cell::Cell;

use crate::model::palette::Palette;
use crate::dialog::add_new_palette_dialog::AddNewPaletteDialog;
use crate::toasts::add_error_toast;
use crate::i18n::i18n;
use crate::util::{model, edit_button_set_visible, edit_button_mode};

use super::palette_row::PaletteRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/palette_page.ui")]
    pub struct PalettePagePriv {
        #[template_child(id = "list_box")]
        pub list_box: TemplateChild<gtk::ListBox>,

        #[template_child(id = "status")]
        pub status: TemplateChild<adw::StatusPage>,

        #[template_child(id = "add_palette_button")]
        pub add_palette_button: TemplateChild<gtk::Button>,

        pub list_store: gio::ListStore,
        pub edit_mode: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PalettePagePriv {
        const NAME: &'static str = "PalettePage";
        type Type = super::PalettePage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                list_box: TemplateChild::default(),
                status: TemplateChild::default(),
                add_palette_button: TemplateChild::default(),
                list_store: gio::ListStore::new(Palette::static_type()),
                edit_mode: Cell::new(false),
            }
        }

    }

    impl ObjectImpl for PalettePagePriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for PalettePagePriv {}
    impl BinImpl for PalettePagePriv {}
    impl PalettePagePriv {}
}

glib::wrapper! {
    pub struct PalettePage(ObjectSubclass<imp::PalettePagePriv>)
    @extends gtk::Widget, adw::Bin;
}


impl PalettePage {
    pub fn new() -> PalettePage {
        let color_panel: PalettePage = glib::Object::builder::<PalettePage>().build();
        color_panel
    }

    fn initialize(&self) {
        let imp = self.imp();

        imp.list_box.bind_model(Some(&imp.list_store), 
        clone!(@strong self as this => @default-panic, move |obj| {
            let palette = obj.clone().downcast::<Palette>().expect("Palette is of wrong type");       
            return PaletteRow::new(palette).upcast::<gtk::Widget>();
            })
        );

        model().connect_local(
            "populated",
            false,
            clone!(@weak self as this => @default-return None, move |_args| {
                this.update_view();
                None
            }),
        );

        imp.add_palette_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.show_new_palette_dialog();
            }),
        );
    }

    fn update_view(&self) {
        let imp = self.imp();
        imp.list_store.remove_all();
        let palettes = model().palettes();
        if palettes.is_empty() {
            imp.status.show();
            imp.list_box.hide();
            edit_button_set_visible(false);
            imp.edit_mode.set(false);
        } else {
            imp.status.hide();
            imp.list_box.show();
            for (_i, palette) in palettes.iter() {
                imp.list_store.append(palette.as_ref());
            }
        }
        self.update_edit_view()
    }

    pub fn toggle_edit_mode(&self) {
        let imp = self.imp();
        if imp.list_store.n_items() > 0 {
            self.set_edit_mode(!imp.edit_mode.get());
        } else {
            add_error_toast(i18n("Cannot toggle edit mode, no palettes added."))
        }
    }

    fn set_edit_mode(&self, mode: bool) {
        let imp = self.imp();
        imp.edit_mode.set(mode);
        edit_button_mode(mode);
        self.update_edit_view();
    }

    fn update_edit_view(&self) {
        let imp = self.imp();
        for row in imp.list_box.observe_children().snapshot() {
            let palette_row = row.downcast::<PaletteRow>().expect("PaletteRow is of wrong type");      
            palette_row.set_edit_mode(imp.edit_mode.get());
        }
    }

    fn show_new_palette_dialog(&self) {
        let dialog = AddNewPaletteDialog::new();
        dialog.connect_response(            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "add" {
                    this.set_edit_mode(false);
                    edit_button_set_visible(true);
                }
            }),
        );
        dialog.show()
    }
}
    