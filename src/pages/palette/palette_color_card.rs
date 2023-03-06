use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, glib::clone, CompositeTemplate};

use std::{cell::RefCell, cell::Cell};

use crate::model::{color::Color, palette::Palette};
use crate::pages::color_square::ColorSquare;
use crate::dialog::delete_color_dialog::DeleteColorDialog;
use crate::util::copy_color;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/palette_color_card.ui")]
    pub struct PaletteColorCardPriv {
        #[template_child(id = "color_bin")]
        pub color_bin: TemplateChild<adw::Bin>,

        #[template_child(id = "hex_label")]
        pub hex_label: TemplateChild<gtk::Label>,

        #[template_child(id = "rgb_label")]
        pub rgb_label: TemplateChild<gtk::Label>,

        #[template_child(id = "button")]
        pub button: TemplateChild<gtk::Button>,

        #[template_child(id = "revealer")]
        pub revealer: TemplateChild<gtk::Revealer>,

        #[template_child(id = "delete_color_button")]
        pub delete_color_button: TemplateChild<gtk::Button>,

        pub color: RefCell<Option<Color>>,
        pub palette: RefCell<Option<Palette>>,
        pub edit_mode: Cell<bool>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaletteColorCardPriv {
        const NAME: &'static str = "PaletteColorCard";
        type Type = super::PaletteColorCard;
        type ParentType = gtk::FlowBoxChild;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PaletteColorCardPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for PaletteColorCardPriv {}
    impl FlowBoxChildImpl for PaletteColorCardPriv {}
    impl PaletteColorCardPriv {}
}

glib::wrapper! {
    pub struct PaletteColorCard(ObjectSubclass<imp::PaletteColorCardPriv>)
    @extends gtk::FlowBoxChild, gtk::Widget;
}


impl PaletteColorCard {
    pub fn new(color: Color, palette: &Palette) -> PaletteColorCard {
        let palette_color_card: PaletteColorCard = glib::Object::builder::<PaletteColorCard>().build();

        palette_color_card.load(color, palette);
        return palette_color_card;
    }

    fn initialize(&self) {
        let imp = self.imp();

        let ctrl = gtk::EventControllerMotion::new();

        ctrl.connect_enter(
            clone!(@strong self as this => move |_controller, _x, _y| {
                if !this.imp().edit_mode.get() {
                    this.imp().button.show();
                } else {
                    this.imp().button.hide();
                }
            })
        );

        ctrl.connect_leave(
            clone!(@strong self as this => move |_controller| {
                this.imp().button.hide();
            })
        );

        self.add_controller(ctrl);

        imp.button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let hex_name = this.imp().color.borrow().as_ref().unwrap().hex_name();
                copy_color(hex_name);
            })
        );

        imp.delete_color_button.connect_clicked(
            clone!(@strong self as this => @default-panic, move |_button| {
                let dialog = DeleteColorDialog::new(this.imp().color.borrow().as_ref().unwrap(), this.imp().palette.borrow().as_ref().unwrap());
                dialog.show();
            })
        );

        imp.edit_mode.set(false);
    }

    fn load(&self, color: Color, palette: &Palette) {
        let imp = self.imp();

        imp.hex_label.set_label(color.hex_name().as_str());
        imp.rgb_label.set_label(color.rgb_name().as_str());

        imp.color_bin.set_child(Some(&ColorSquare::new(110, color.rgb_name())));

        imp.color.replace(Some(color));
        imp.palette.replace(Some(palette.clone()));
    }

    pub fn set_edit_mode(&self, mode: bool) {
        let imp = self.imp();

        imp.edit_mode.set(mode);
        self.update_edit_view();
    }

    fn update_edit_view(&self) {
        let imp = self.imp();

        imp.revealer.set_reveal_child(imp.edit_mode.get());
        imp.button.set_visible(false);
    }
}
