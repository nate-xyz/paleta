/* add_new_palette_dialog.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk, glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::model::color::Color;
use crate::pages::color_square::ColorSquare;
use crate::toasts::{add_error_toast, add_success_toast};
use crate::i18n::{i18n, i18n_k};
use crate::util::{model, database, active_window, rgb_to_hex};

use super::simple_palette_row::SimplePaletteRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/add_new_palette_dialog.ui")]
    pub struct AddNewPaletteDialogPriv {
        #[template_child(id = "adw_entry_row")]
        pub adw_entry_row: TemplateChild<adw::EntryRow>,

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
        pub color: RefCell<Option<Color>>,
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddNewPaletteDialogPriv {
        const NAME: &'static str = "AddNewPaletteDialog";
        type Type = super::AddNewPaletteDialog;
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
                color_selection_row: TemplateChild::default(),
                picker_button: TemplateChild::default(),
                currently_selected_label: TemplateChild::default(),
                currently_selected_color_square: TemplateChild::default(),
                revealer: TemplateChild::default(),
                color_instruction_label: TemplateChild::default(),
                color_chooser: RefCell::new(None),
                color: RefCell::new(None),
                name: RefCell::new(String::new()),
            }
        }
    }

    impl ObjectImpl for AddNewPaletteDialogPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for AddNewPaletteDialogPriv {}
    impl WindowImpl for AddNewPaletteDialogPriv {}
    impl MessageDialogImpl for AddNewPaletteDialogPriv {}
    impl AddNewPaletteDialogPriv {}
}

glib::wrapper! {
    pub struct AddNewPaletteDialog(ObjectSubclass<imp::AddNewPaletteDialogPriv>)
    @extends gtk::Widget, gtk::Window, adw::MessageDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AddNewPaletteDialog {
    pub fn new() -> AddNewPaletteDialog {
        let save_dialog: AddNewPaletteDialog = glib::Object::builder::<AddNewPaletteDialog>().build();
        save_dialog
    }

    fn initialize(&self) {
        let imp = self.imp();
        let palette_index = database().query_n_palettes()+1;

        self.set_transient_for(Some(&active_window().unwrap()));
        self.set_name(i18n_k("Palette #{palette_index}", &[("palette_index", &palette_index.to_string())]));

        self.init_color_chooser();
        self.connect_response(
            None,
            clone!(@strong self as this => move |_dialog, response| {
                if response == "add" {
                    this.add_new_palette();
                }
            }),
        );
        imp.picker_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                this.imp().color_chooser.borrow().as_ref().unwrap().show();
            }),
        );
        if model().colors().len() > 0 {
            let simple_palette_row = SimplePaletteRow::new();
            simple_palette_row.connect_local(
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
            imp.color_selection_row.set_child(Some(&simple_palette_row));
        } else {
            imp.color_instruction_label.set_label(&i18n("Pick a new color to add to new palette."));
            
        }
    }

    fn set_name(&self, name: String) {
        let imp = self.imp();
        imp.adw_entry_row.set_text(name.as_str());
        imp.name.replace(name);
    }

    fn set_current_color(&self, color: Color) {
        let imp = self.imp();
        imp.revealer.set_reveal_child(false);
        imp.currently_selected_label.set_label(&i18n_k("Currently selected color: {color_hex}", &[("color_hex", &color.hex_name())]));
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
            .title(&i18n("Choose color to add to new palette"))
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

    fn add_new_palette(&self) {
        let imp = self.imp();

        if let Some(color) = imp.color.borrow().as_ref() {
            let mut name = imp.adw_entry_row.text().to_string();
            if name == "" {
                name = imp.name.borrow().clone();
            }

            if database().add_palette_new(name.clone(), color.hex_name(), color.rgba()) {
                add_success_toast(&i18n("Created!"), &i18n_k("New palette: «{palette_name}»", &[("palette_name", &name)]));
            } else {
                add_error_toast(i18n_k("Unable to add new palette «{palette_name}»", &[("palette_name", &name)]));
            }
            return;
        }
        add_error_toast(i18n("Unable to add palette, must select a color."));
    }

}
