use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};

use std::{cell::RefCell, rc::Rc, collections::HashMap};
use log::{debug, error};

use crate::database::Database;

use super::color::Color;
use super::palette::Palette;

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default)]
    pub struct ModelPriv {
        database: RefCell<Rc<Database>>,
        colors: RefCell<Option<HashMap<i64, Rc<Color>>>>,
        palettes: RefCell<Option<HashMap<i64, Rc<Palette>>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModelPriv {
        const NAME: &'static str = "Model";
        type Type = super::Model;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ModelPriv {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("populated").build(),
                ]
            });
            
            SIGNALS.as_ref()
        }
    }

    impl ModelPriv {
        pub fn load_db(&self, database: Rc<Database>) {
            *self.database.borrow_mut() = Rc::clone(&database);
        }

        pub fn reset_model(&self) {
            *self.colors.borrow_mut() = Some(HashMap::new());
            *self.palettes.borrow_mut() = Some(HashMap::new());
        }

        // ######
        // # POPULATE APP MODEL from DATABASE
        // ######

        pub fn populate(&self) {
            debug!("Inside model populate");
            self.reset_model();
            self.populate_colors();
            self.populate_palettes();
            self.populate_color_palette_join();
            self.obj().emit_by_name::<()>("populated", &[]);
        }



        fn populate_colors(&self) {
            debug!("populate color");
            let result = self.database.borrow().query_colors();
            match result {
                Ok(list) => {
                    if list.is_empty() {
                        debug!("color list empty");
                        return;
                    }
                    let mut color_map = HashMap::new();
                    for (id, r, g, b, a, hex) in list {
                        let color = Rc::new(Color::new(id, r, g, b, a, hex));
                        color_map.insert(id, color);
                    }
                    self.colors.replace(Some(color_map));
                }
                Err(e) => error!("An error occurred: {}", e),
            }
        }

        fn populate_palettes(&self) {
            debug!("populate color");
            let result = self.database.borrow().query_palettes();
            match result {
                Ok(list) => {
                    if list.is_empty() {
                        debug!("palette list empty");
                        return;
                    }
                    let mut palette_map = HashMap::new();
                    for (id, name) in list {
                        let palette = Rc::new(Palette::new(id, name));
                        palette_map.insert(id, palette);
                    }
                    self.palettes.replace(Some(palette_map));
                }
                Err(e) => error!("An error occurred: {}", e),
            }
        }

        fn populate_color_palette_join(&self) {
            debug!("populate color");
            let result = self.database.borrow().query_palette_color_junction();
            match result {
                Ok(list) => {
                    if list.is_empty() {
                        debug!("join list empty");
                        return;
                    }

                    for (_id, palette_id, color_id) in list {
                        if let Ok(palette) = self.palette(palette_id) {
                            if let Ok(color) = self.color(color_id) {
                                palette.add_color(color)
                            }
                        }
                    }

                }
                Err(e) => error!("An error occurred: {}", e),
            }
        }


        pub fn colors(&self) -> HashMap<i64, Rc<Color>> {
            self.colors.borrow().as_ref().unwrap().clone()
        }

        pub fn palettes(&self) -> HashMap<i64, Rc<Palette>> {
            self.palettes.borrow().as_ref().unwrap().clone()
        }

        pub fn color(&self, id: i64) -> Result<Rc<Color>, String> {
            match self.colors.borrow().as_ref() {
                Some(map) => match map.get(&id) {
                    Some(color) => return Ok(color.clone()),
                    None => return Err("id not in map".to_string()),
                },
                None => return Err("hashmap does not exist".to_string()),
            }
        }

        pub fn palette(&self, id: i64) -> Result<Rc<Palette>, String> {
            match self.palettes.borrow().as_ref() {
                Some(map) => match map.get(&id) {
                    Some(palette) => return Ok(palette.clone()),
                    None => return Err("id not in map".to_string()),
                },
                None => return Err("hashmap does not exist".to_string()),
            }
        }

    }
}

glib::wrapper! {
    pub struct Model(ObjectSubclass<imp::ModelPriv>);
}

impl Default for Model {
    fn default() -> Self {
        glib::Object::builder::<Model>().build()
    }
}

impl Model {
    pub fn new() -> Model {
        Self::default()
    }

    pub fn load_db(&self, database: Rc<Database>) {
        self.imp().load_db(database);
    }

    pub fn populate_all(&self) {
        self.imp().populate();
    }
    
    pub fn colors(&self) -> HashMap<i64, Rc<Color>> {
        self.imp().colors()
    }

    pub fn palettes(&self) -> HashMap<i64, Rc<Palette>> {
        self.imp().palettes()
    }

    pub fn color(&self, id: i64) -> Result<Rc<Color>, String> {
        self.imp().color(id)
    }

    pub fn palette(&self, id: i64) -> Result<Rc<Palette>, String> {
        self.imp().palette(id)
    }
}
