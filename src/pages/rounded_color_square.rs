use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::{gdk, glib, graphene, gsk};
use std::cell::{RefCell, Cell};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RoundedColorSquare {
        pub size: Cell<i32>,
        pub color_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RoundedColorSquare {
        const NAME: &'static str = "RoundedColorSquare";
        type Type = super::RoundedColorSquare;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for RoundedColorSquare {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for RoundedColorSquare {
        fn measure(&self, _orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            (self.size.get(), self.size.get(), -1, -1)
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = &*self.obj();
            let color = gdk::RGBA::parse(&self.color_name.borrow().clone()).unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32);

            let rounded_rect = gsk::RoundedRect::from_rect(rect, 10.0);
            snapshot.push_rounded_clip(&rounded_rect);
            snapshot.append_color(&color, &rect);
            snapshot.pop();
        }
    }

}

glib::wrapper! {
    pub struct RoundedColorSquare(ObjectSubclass<imp::RoundedColorSquare>)
        @extends gtk::Widget;
}

impl RoundedColorSquare {
    pub fn new(size: i32, color_name: String) -> RoundedColorSquare {
        let cs: RoundedColorSquare = glib::Object::builder::<RoundedColorSquare>().build();
        cs.load(size, color_name);
        cs
    }

    fn load(&self, size: i32, color_name: String) {
        self.imp().size.set(size);
        self.imp().color_name.replace(color_name);
    }
}