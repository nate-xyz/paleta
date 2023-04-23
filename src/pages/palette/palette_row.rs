use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib, glib::clone, CompositeTemplate};

use std::cell::{RefCell, Cell};

use crate::model::{
    palette::Palette,
    color::Color,
};
use crate::dialog::{
    rename_palette_dialog::RenamePaletteDialog,
    duplicate_palette_dialog::DuplicatePaletteDialog,
    delete_palette_dialog::DeletePaletteDialog,
    add_color_dialog::AddColorDialog,
};

use super::palette_color_card::PaletteColorCard;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/palette_row.ui")]
    pub struct PaletteRowPriv {
        #[template_child(id = "flow_box")]
        pub flow_box: TemplateChild<gtk::FlowBox>,

        #[template_child(id = "title_label")]
        pub title_label: TemplateChild<gtk::Label>,

        #[template_child(id = "edit_mode_revealer")]
        pub edit_mode_revealer: TemplateChild<gtk::Revealer>,

        #[template_child(id = "edit_name_button")]
        pub edit_name_button: TemplateChild<gtk::Button>,

        #[template_child(id = "duplicate_palette_button")]
        pub duplicate_palette_button: TemplateChild<gtk::Button>,

        #[template_child(id = "add_color_button")]
        pub add_color_button: TemplateChild<gtk::Button>,

        #[template_child(id = "delete_palette_button")]
        pub delete_palette_button: TemplateChild<gtk::Button>,

        pub list_store: gio::ListStore,
        pub edit_mode: Cell<bool>,
        pub palette: RefCell<Option<Palette>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaletteRowPriv {
        const NAME: &'static str = "PaletteRow";
        type Type = super::PaletteRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                flow_box: TemplateChild::default(),
                title_label: TemplateChild::default(),
                edit_mode_revealer: TemplateChild::default(),
                edit_name_button: TemplateChild::default(),
                duplicate_palette_button: TemplateChild::default(),
                add_color_button: TemplateChild::default(),
                delete_palette_button: TemplateChild::default(),
                list_store: gio::ListStore::new(Color::static_type()),
                edit_mode: Cell::new(false),
                palette: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for PaletteRowPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for PaletteRowPriv {}
    impl ListBoxRowImpl for PaletteRowPriv {}
    impl PaletteRowPriv {}
}

glib::wrapper! {
    pub struct PaletteRow(ObjectSubclass<imp::PaletteRowPriv>)
    @extends gtk::Widget, gtk::ListBoxRow;
}

impl PaletteRow {
    pub fn new(palette: Palette) -> PaletteRow {
        let palette_row: PaletteRow = glib::Object::builder::<PaletteRow>().build();
        palette_row.load(palette);
        palette_row
    }

    fn initialize(&self) {
        let imp = self.imp();
        imp.flow_box.bind_model(Some(&imp.list_store), 
        clone!(@strong self as this => @default-panic, move |obj| {
            let color = obj.clone().downcast::<Color>().expect("Palette is of wrong type");       
            return PaletteColorCard::new(color, this.imp().palette.borrow().as_ref().unwrap()).upcast::<gtk::Widget>();
            })
        );
        
        imp.edit_name_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let dialog = RenamePaletteDialog::new(this.imp().palette.borrow().as_ref().unwrap());
                dialog.show();
            })
        );

        imp.duplicate_palette_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let dialog = DuplicatePaletteDialog::new(this.imp().palette.borrow().as_ref().unwrap());
                dialog.show();
            })
        );

        imp.delete_palette_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let dialog = DeletePaletteDialog::new(this.imp().palette.borrow().as_ref().unwrap());
                dialog.show();
            })
        );

        imp.add_color_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let dialog = AddColorDialog::new(this.imp().palette.borrow().as_ref().unwrap());
                dialog.show();
            })
        );
    }

    fn load(&self, palette: Palette) {
        let imp = self.imp();
        imp.title_label.set_label(palette.name().as_str());
        imp.flow_box.set_min_children_per_line(palette.len());
        imp.flow_box.set_max_children_per_line(palette.len());
        imp.palette.replace(Some(palette));
        self.update_view();
    }

    fn update_view(&self) {
        let imp = self.imp();
        imp.list_store.remove_all();
        for color in imp.palette.borrow().as_ref().unwrap().colors().unwrap() {
            imp.list_store.append(color.as_ref());
        }
        self.update_edit_view();
    }

    pub fn set_edit_mode(&self, mode: bool) {
        let imp = self.imp();
        imp.edit_mode.set(mode);
        for row in imp.flow_box.observe_children().snapshot() {
            let palette_card = row.downcast::<PaletteColorCard>().expect("PaletteRow is of wrong type");      
            palette_card.set_edit_mode(imp.edit_mode.get());
        }
        self.update_edit_view();
    }

    fn update_edit_view(&self) {
        let imp = self.imp();
        imp.edit_mode_revealer.set_reveal_child(imp.edit_mode.get())
    }
}
