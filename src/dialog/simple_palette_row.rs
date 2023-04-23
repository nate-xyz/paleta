/* simple_palette_row.rs
 *
 * SPDX-FileCopyrightText: 2023 nate-xyz
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib, glib::clone, CompositeTemplate};

use crate::model::color::Color;
use crate::util::model;

use super::simple_color_card::SimpleColorCard;

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/nate_xyz/Paleta/simple_palette_row.ui")]
    pub struct SimplePaletteRowPriv {
        #[template_child(id = "flow_box")]
        pub flow_box: TemplateChild<gtk::FlowBox>,

        pub list_store: gio::ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimplePaletteRowPriv {
        const NAME: &'static str = "SimplePaletteRow";
        type Type = super::SimplePaletteRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                flow_box: TemplateChild::default(),
                list_store: gio::ListStore::new(Color::static_type()),
            }
        }
    }

    impl ObjectImpl for SimplePaletteRowPriv {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("color-selected").param_types([<Color>::static_type()]).build()]);

            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for SimplePaletteRowPriv {}
    impl ListBoxRowImpl for SimplePaletteRowPriv {}
    impl SimplePaletteRowPriv {}
}

glib::wrapper! {
    pub struct SimplePaletteRow(ObjectSubclass<imp::SimplePaletteRowPriv>)
    @extends gtk::Widget, gtk::ListBoxRow;
}

impl SimplePaletteRow {
    pub fn new() -> SimplePaletteRow {
        let palette_row: SimplePaletteRow = glib::Object::builder::<SimplePaletteRow>().build();
        palette_row.initialize();
        palette_row
    }

    fn initialize(&self) {
        let imp = self.imp();
        imp.flow_box.bind_model(Some(&imp.list_store), 
        clone!(@strong self as this => @default-panic, move |obj| {
            let color = obj.clone().downcast::<Color>().expect("Palette is of wrong type");     
            let color_card = SimpleColorCard::new(color).upcast::<gtk::Widget>();
            color_card.connect_local(
                "color-selected",
                false,
                clone!(@weak this => @default-return None, move |value| {
                    if let Some(color_val) = value.get(1) {
                        if let Some(color) = color_val.get::<Color>().ok() {
                            this.emit_by_name::<()>("color-selected", &[&color]);
                        }                        
                    }
                    None
                }),
            );
            return color_card;
            })
        );

        self.update_view();
    }

    fn update_view(&self) {
        let imp = self.imp();
        let colors = model().colors();
        let n_colors = colors.len() as u32;
        imp.flow_box.set_min_children_per_line(n_colors);
        imp.flow_box.set_max_children_per_line(n_colors);
        imp.list_store.remove_all();
        for (_i, color) in colors.iter() {
            imp.list_store.append(color.as_ref());
        }
    }
}
