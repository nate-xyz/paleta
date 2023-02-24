use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, glib, glib::clone, CompositeTemplate};

use std::cell::RefCell;
use log::error;

use super::extracted_color::ExtractedColor;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/extracted_color_row.ui")]
    pub struct ExtractedColorRowPriv {
        
        #[template_child(id = "row_box")]
        pub row_box: TemplateChild<gtk::Box>,

        #[template_child(id = "hex_name_label")]
        pub hex_name_label: TemplateChild<gtk::Label>,

        #[template_child(id = "rgb_name_label")]
        pub rgb_name_label: TemplateChild<gtk::Label>,

        #[template_child(id = "copy_icon")]
        pub copy_icon: TemplateChild<gtk::Image>,

        pub hex_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExtractedColorRowPriv {
        const NAME: &'static str = "ExtractedColorRow";
        type Type = super::ExtractedColorRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExtractedColorRowPriv {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().initialize();
        }
    }

    impl WidgetImpl for ExtractedColorRowPriv {}
    impl ListBoxRowImpl for ExtractedColorRowPriv {}
    impl ExtractedColorRowPriv {}
}

glib::wrapper! {
    pub struct ExtractedColorRow(ObjectSubclass<imp::ExtractedColorRowPriv>)
    @extends gtk::Widget, gtk::ListBoxRow;
}

impl ExtractedColorRow {
    pub fn new(color: &ExtractedColor) -> ExtractedColorRow {
        let color_row: ExtractedColorRow = glib::Object::builder::<ExtractedColorRow>().build();
        color_row.load_color(color);
        color_row
    }

    fn initialize(&self) {
        let ctrl = gtk::EventControllerMotion::new();
        ctrl.connect_enter(clone!(@strong self as this => move |_controller, _x, _y| {
            this.imp().copy_icon.show();
        }));
        ctrl.connect_leave(clone!(@strong self as this => move |_controller| {
            this.imp().copy_icon.hide();
        }));
        self.add_controller(ctrl);
    }


    fn load_color(&self, color: &ExtractedColor) {
        let imp = self.imp();
        imp.rgb_name_label.set_label(color.rgb_name().as_str());
        imp.hex_name_label.set_label(color.hex_name().as_str());
        imp.hex_name.replace(color.hex_name());
        match gdk::RGBA::parse(color.rgb_name()) {
            Ok(rgba) => {
                let color_button = gtk::ColorButton::with_rgba(&rgba);
                color_button.set_show_editor(true);
                imp.row_box.prepend(&color_button);
            },
            Err(e) => error!("{}", e),
        }
    }

    pub fn hex_name(&self) -> String {
        self.imp().hex_name.borrow().clone()
    }

}
