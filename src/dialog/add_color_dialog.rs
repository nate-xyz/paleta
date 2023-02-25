use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{gdk, glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;
use log::debug;

use crate::util::{model, database, active_window, rgb_to_hex};
use crate::toasts::{add_error_toast, add_color_toast};
use crate::i18n::{i18n, i18n_f};

use crate::model::palette::Palette;
use crate::model::color::Color;
use crate::pages::color_square::ColorSquare;

use super::simple_palette_row::SimplePaletteRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/add_color_dialog.ui")]
    pub struct AddColorDialogPriv {
        #[template_child(id = "color_selection_row")]
        pub color_selection_row: TemplateChild<adw::Bin>,

        #[template_child(id = "picker_button")]
        pub picker_button: TemplateChild<gtk::Button>,

        #[template_child(id = "currently_selected_label")]
        pub currently_selected_label: TemplateChild<gtk::Label>,

        #[template_child(id = "currently_selected_color_square")]
        pub currently_selected_color_square: TemplateChild<adw::Bin>,

        #[template_child(id = "revealer")]
        pub revealer: TemplateChild<gtk::Revealer>,

        #[template_child(id = "color_instruction_label")]
        pub color_instruction_label: TemplateChild<gtk::Label>,


        pub color_chooser: RefCell<Option<gtk::ColorChooserDialog>>,
        pub palette: RefCell<Option<Palette>>,
        pub color: RefCell<Option<Color>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddColorDialogPriv {
        const NAME: &'static str = "AddColorDialog";
        type Type = super::AddColorDialog;
        type ParentType = adw::MessageDialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                color_selection_row: TemplateChild::default(),
                picker_button: TemplateChild::default(),
                currently_selected_label: TemplateChild::default(),
                currently_selected_color_square: TemplateChild::default(),
                revealer: TemplateChild::default(),
                color_instruction_label: TemplateChild::default(),
                color_chooser: RefCell::new(None),
                palette: RefCell::new(None),
                color: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for AddColorDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for AddColorDialogPriv {}
    impl WindowImpl for AddColorDialogPriv {}
    impl MessageDialogImpl for AddColorDialogPriv {}
    impl AddColorDialogPriv {}
}

glib::wrapper! {
    pub struct AddColorDialog(ObjectSubclass<imp::AddColorDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AddColorDialog {
    pub fn new(palette: &Palette) -> AddColorDialog {
        let save_dialog: AddColorDialog = glib::Object::builder::<AddColorDialog>().build();
        save_dialog.load(palette);
        save_dialog
    }

    fn initialize(&self) {
        let imp = self.imp();
        
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "add" {
                    this.add_color();
                }
            }),
        );
        self.set_transient_for(Some(&active_window().unwrap()));
        imp.picker_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.imp().color_chooser.borrow().as_ref().unwrap().show();
            }),
        );
    }

    fn load(&self, palette: &Palette) {
        debug!("load palette");
        self.set_heading(Some(&i18n_f("Add Color to {}", &[&palette.name()])));
        let imp = self.imp();
        if model().colors().len() > 0 {
            let simple_row = SimplePaletteRow::new();
            simple_row.connect_local(
                "color-selected",
                false,
                clone!(@weak self as this => @default-return None, move |value| {
                    let color_val = value.get(1); 
                    match color_val {
                        Some(color_val) => {
                            let color = color_val.get::<Color>().ok().unwrap();
                            this.set_current_color(color);
                        },
                        None => (),
                    }
                    
                    None
                }),
            );
            imp.color_selection_row.set_child(Some(&simple_row));
        } else {
            imp.color_instruction_label.set_label(&i18n_f("Pick a new color to add to {}.", &[&palette.name()]));

            
        }
        imp.palette.replace(Some(palette.clone()));
        self.init_color_chooser();
    }

    fn set_current_color(&self, color: Color) {
        let imp = self.imp();
        imp.revealer.set_reveal_child(false);
        imp.currently_selected_label.set_label(&i18n_f("Currently selected color: {}", &[&color.hex_name()]));

        imp.currently_selected_color_square.set_child(Some(&ColorSquare::new(110, color.rgb_name())));
        imp.color.replace(Some(color));
        if !imp.revealer.reveals_child() {
            imp.revealer.set_reveal_child(true);
        }           
    }

    fn chooser_response(&self, color: gdk::RGBA) {
        let red = (color.red() * 255.0) as i64;
        let green = (color.green() * 255.0) as i64;
        let blue = (color.blue() * 255.0) as i64;
        let alpha = color.red() as f64;
        let hex = rgb_to_hex(red as u8, green as u8, blue as u8);
        self.set_current_color(Color::new(-1, red, green, blue, alpha, hex))
    }


    fn init_color_chooser(&self) {
        let dialog = gtk::ColorChooserDialog::builder()
        .title(&i18n_f("Choose new color to add to {}", &[&self.palette_name()]))
        .transient_for(self)
        .build();


        dialog.connect_response(
            clone!(@strong dialog, @weak self as this => move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    this.chooser_response(dialog.rgba());
                }
                dialog.close();
                this.init_color_chooser();
            }),
        );

        self.imp().color_chooser.replace(Some(dialog));
    }

    fn palette_name(&self) -> String {
        self.imp().palette.borrow().as_ref().unwrap().name()
    }

    fn add_color(&self) {
        let imp = self.imp();
        match imp.palette.borrow().as_ref() {
            Some(palette) => {
                match imp.color.borrow().as_ref() {
                    Some(color) => {
                        if database().add_color_to_palette(palette.id(), color.hex_name(), color.rgba()) {
                            add_color_toast(color.hex_name(), palette.name());
                            return;
                        } else {
                            add_error_toast(i18n_f("Unable to add color {}.", &[&color.hex_name()]));
                        }
                    },
                    None => add_error_toast(i18n("Unable to add color.")),
                }
            },
            None => add_error_toast(i18n("Unable to add color.")),
        }
    }

}
