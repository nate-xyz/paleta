use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk::{glib, CompositeTemplate};

use crate::model::color::Color;
use crate::pages::color_square::ColorSquare;

mod imp {
    use super::*;
    
    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/simpler_delete_color_card.ui")]
    pub struct SimplerDeleteColorCardPriv {
        #[template_child(id = "color_bin")]
        pub color_bin: TemplateChild<adw::Bin>,
        
        #[template_child(id = "hex_label")]
        pub hex_label: TemplateChild<gtk::Label>,

        #[template_child(id = "rgb_label")]
        pub rgb_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimplerDeleteColorCardPriv {
        const NAME: &'static str = "SimplerDeleteColorCard";
        type Type = super::SimplerDeleteColorCard;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SimplerDeleteColorCardPriv {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for SimplerDeleteColorCardPriv {}
    impl BinImpl for SimplerDeleteColorCardPriv {}
    impl SimplerDeleteColorCardPriv {}
}

glib::wrapper! {
    pub struct SimplerDeleteColorCard(ObjectSubclass<imp::SimplerDeleteColorCardPriv>)
    @extends gtk::Widget, adw::Bin;
}


impl SimplerDeleteColorCard {
    pub fn new(color: &Color) -> SimplerDeleteColorCard {
        let palette_color_card: SimplerDeleteColorCard = glib::Object::builder::<SimplerDeleteColorCard>().build();
        palette_color_card.load(color);
        palette_color_card
    }

    fn load(&self, color: &Color) {
        let imp = self.imp();
        imp.hex_label.set_label(color.hex_name().as_str());
        imp.rgb_label.set_label(color.rgb_name().as_str());
        imp.color_bin.set_child(Some(&ColorSquare::new(110, color.rgb_name())));
    }

}
    