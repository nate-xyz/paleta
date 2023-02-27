use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::pages::rounded_color_square::RoundedColorSquare;

use super::extracted_color::ExtractedColor;
use crate::util::copy_color;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/extracted_color_card.ui")]
    pub struct ExtractedColorCard  {
        #[template_child(id = "color_bin")]
        pub color_bin: TemplateChild<adw::Bin>,
        
        #[template_child(id = "color_label")]
        pub color_label: TemplateChild<gtk::Label>,

        #[template_child(id = "button")]
        pub button: TemplateChild<gtk::Button>,

        pub color: RefCell<Option<ExtractedColor>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExtractedColorCard {
        const NAME: &'static str = "ExtractedColorCard";
        type Type = super::ExtractedColorCard;
        type ParentType = gtk::FlowBoxChild;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExtractedColorCard {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for ExtractedColorCard {}
    impl FlowBoxChildImpl for ExtractedColorCard {}
    impl ExtractedColorCard {}

}

glib::wrapper! {
    pub struct ExtractedColorCard(ObjectSubclass<imp::ExtractedColorCard>)
    @extends gtk::FlowBoxChild, gtk::Widget;
}

impl ExtractedColorCard {
    pub fn new(color: &ExtractedColor) -> ExtractedColorCard {
        let extracted_color_card: ExtractedColorCard = glib::Object::builder::<ExtractedColorCard>().build();
        extracted_color_card.load(color);
        extracted_color_card
    }

    fn initialize(&self) {
        let imp = self.imp();
        
        let ctrl = gtk::EventControllerMotion::new();
        ctrl.connect_enter(clone!(@strong self as this => move |_controller, _x, _y| {
            this.imp().button.show();
        }));
        ctrl.connect_leave(clone!(@strong self as this => move |_controller| {
            this.imp().button.hide();
        }));
        self.add_controller(ctrl);

        imp.button.connect_clicked(clone!(@strong self as this => @default-panic, move |_button| {
            let hex_name = this.imp().color.borrow().as_ref().unwrap().hex_name();
            copy_color(hex_name);
        }));
    }

    fn load(&self, color: &ExtractedColor) {
        let imp = self.imp();
        imp.color_bin.set_child(Some(&RoundedColorSquare::new(110, color.hex_name())));
        
        if color.is_light() {
            imp.color_label.set_label(&format!("<span foreground=\"#404040\" style=\"oblique\" size=\"x-large\">{}</span>", color.hex_name()));
        } else {
            imp.color_label.set_label(&format!("<span foreground=\"#BFBFBF\" style=\"oblique\" size=\"x-large\">{}</span>", color.hex_name()));
        }

        imp.color.replace(Some(color.clone()));
    }
}