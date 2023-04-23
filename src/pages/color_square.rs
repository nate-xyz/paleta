use gtk::{prelude::*, subclass::prelude::*, gdk, glib, graphene};

use std::cell::{RefCell, Cell};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ColorSquare {
        pub size: Cell<i32>,
        pub rgb_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorSquare {
        const NAME: &'static str = "ColorSquare";
        type Type = super::ColorSquare;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for ColorSquare {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for ColorSquare {
        fn measure(&self, _orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            (self.size.get(), self.size.get(), -1, -1)
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = &*self.obj();
            let color = gdk::RGBA::parse(&self.rgb_name.borrow().clone()).unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32);
            snapshot.append_color(&color, &rect);
        }
    }

}

glib::wrapper! {
    pub struct ColorSquare(ObjectSubclass<imp::ColorSquare>)
        @extends gtk::Widget;
}

impl ColorSquare {
    pub fn new(size: i32, rgb_name: String) -> ColorSquare {
        let cs: ColorSquare = glib::Object::builder::<ColorSquare>().build();
        cs.load(size, rgb_name);
        cs
    }

    fn load(&self, size: i32, rgb_name: String) {
        self.imp().size.set(size);
        self.imp().rgb_name.replace(rgb_name);
    }
}