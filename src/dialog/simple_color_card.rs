use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;

use crate::model::color::Color;
use crate::pages::color_square::ColorSquare;

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/simple_color_card.ui")]
    pub struct SimpleColorCardPriv {
        #[template_child(id = "color_bin")]
        pub color_bin: TemplateChild<adw::Bin>,

        #[template_child(id = "hex_label")]
        pub hex_label: TemplateChild<gtk::Label>,

        #[template_child(id = "rgb_label")]
        pub rgb_label: TemplateChild<gtk::Label>,

        #[template_child(id = "button")]
        pub button: TemplateChild<gtk::Button>,

        pub color: RefCell<Option<Color>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleColorCardPriv {
        const NAME: &'static str = "SimpleColorCard";
        type Type = super::SimpleColorCard;
        type ParentType = gtk::FlowBoxChild;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SimpleColorCardPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("color-selected").param_types([<Color>::static_type()]).build()]);

            return SIGNALS.as_ref();
        }
    }

    impl WidgetImpl for SimpleColorCardPriv {}
    impl FlowBoxChildImpl for SimpleColorCardPriv {}
    impl SimpleColorCardPriv {}
}

glib::wrapper! {
    pub struct SimpleColorCard(ObjectSubclass<imp::SimpleColorCardPriv>)
    @extends gtk::FlowBoxChild, gtk::Widget;
}

impl SimpleColorCard {
    pub fn new(color: Color) -> SimpleColorCard {
        let palette_color_card: SimpleColorCard = glib::Object::builder::<SimpleColorCard>().build();

        palette_color_card.load(color);
        return palette_color_card;
    }

    fn initialize(&self) {
        let imp = self.imp();

        let ctrl = gtk::EventControllerMotion::new();

        ctrl.connect_enter(
            clone!(@strong self as this => move |_controller, _x, _y| {
                this.imp().button.show();
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
                this.emit_by_name::<()>("color-selected", &[this.imp().color.borrow().as_ref().unwrap()]);
            })
        );
    }

    fn load(&self, color: Color) {
        let imp = self.imp();

        imp.hex_label.set_label(color.hex_name().as_str());
        imp.rgb_label.set_label(color.rgb_name().as_str());
        imp.color_bin.set_child(Some(&ColorSquare::new(110, color.rgb_name())));

        imp.color.replace(Some(color));
    }
}
