use gtk::glib;
use gtk::subclass::prelude::*;

use std::{cell::Cell, cell::RefCell, rc::Rc};

use super::color::Color;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PalettePriv {
        pub id: Cell<i64>,
        pub name: RefCell<String>,
        pub colors: RefCell<Option<Vec<Rc<Color>>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PalettePriv {
        const NAME: &'static str = "Palette";
        type Type = super::Palette;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for PalettePriv {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct Palette(ObjectSubclass<imp::PalettePriv>);
}

impl Palette {
    pub fn new(id: i64, name: String) -> Palette {
        let palette: Palette = glib::Object::builder::<Palette>().build();
        palette.load(id, name);
        palette
    }

    fn load(&self, id: i64, name: String) {
        let imp = self.imp();
        imp.id.set(id);
        imp.name.replace(name);
    }

    pub fn id(&self) -> i64 {
        self.imp().id.get()
    }

    pub fn name(&self) -> String {
        self.imp().name.borrow().clone()
    }

    pub fn add_color(&self, color: Rc<Color>) {
        let imp = self.imp();

        if None == imp.colors.borrow().as_ref() {
            imp.colors.replace(Some(vec![color]));
            return;
        } 

        if let Some(colors)  = imp.colors.borrow_mut().as_mut() {
            colors.push(color);
            return;
        }

    }

    pub fn colors(&self) -> Option<Vec<Rc<Color>>> {
        if let Some(color) = self.imp().colors.borrow().as_ref() {
            Some(color.clone())
        } else {
            None
        }
    }


    pub fn len(&self) -> u32 {
        if let Some(color) = self.imp().colors.borrow().as_ref() {
            color.len() as u32
        } else {
            0
        }
    }
}
    