use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};

use std::{env, fs, cell::RefCell, path::PathBuf, error::Error};
use log::{debug, error};
use rusqlite::{params, Connection, Result, Transaction};
use directories_next::BaseDirs;

use crate::pages::image_drop::extracted_color::ExtractedColor;

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default)]
    pub struct DatabasePriv {
        pub conn: RefCell<Option<Connection>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DatabasePriv {
        const NAME: &'static str = "Database";
        type Type = super::Database;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for DatabasePriv {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("populate-model").build()]);

            SIGNALS.as_ref()
        }
    }
}

glib::wrapper! {
    pub struct Database(ObjectSubclass<imp::DatabasePriv>);
}

impl Default for Database {
    fn default() -> Self {
        glib::Object::builder::<Database>().build()
    }
}

impl Database {
    pub fn new() -> Database {
        let database: Database = Self::default();

        let path = env::current_dir().ok().unwrap();
        debug!("The current directory is {}", path.display());

        database
    }

    pub fn try_loading_database(&self) -> bool {
        if self.open_connection_to_db() {
            self.emit_by_name::<()>("populate-model", &[]);
            return true
        } else {
            debug!("unable to open database");
            return false
        }
    }
    
    fn open_connection_to_db(&self) -> bool {
        let path = self.database_location();
        let conn = match Connection::open_with_flags(
            path.clone(),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
        ) {
            Ok(c) => {
                debug!("open database by existing uri");
                c
            }
            Err(_) => {
                debug!("open NEW database");
                Connection::open(path).unwrap()
            }
        };

        let result = conn.query_row("SELECT sqlite_source_id()", params![], |row| row.get(0));

        debug!(
            "sqlite_source_id: {}",
            result.unwrap_or("Error".to_string())
        );

        *self.imp().conn.borrow_mut() = Some(conn);

        let result = self.setup_db_tables();
        match result {
            Ok(()) => {
                return true;
            }
            Err(err) => {
                error!("An error occurred: {}", err);
                return false;
            }
        }
    }

    fn database_location(&self) -> PathBuf {
        if let Some(base_dirs) = BaseDirs::new() {
            let folder = base_dirs.data_dir().join("io.github.nate_xyz.Paleta");
            fs::create_dir_all(folder.clone()).unwrap();
            folder.join("paleta_database.db")
        } else {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("io.github.nate_xyz.Paleta").unwrap();
            match xdg_dirs.place_data_file("paleta_database.db") {
                Ok(path) => {
                    let folder = xdg_dirs.get_data_home();
                    fs::create_dir_all(folder).unwrap();
                    path
                }
                Err(_) => PathBuf::from("paleta_database.db"),
            }
        }
    }

    fn setup_db_tables(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let connection = conn.as_ref().ok_or("Connection not established")?;

        connection
            .execute("PRAGMA foreign_keys = ON", params![])
            .unwrap();

        connection.execute("CREATE TABLE IF NOT EXISTS
        Colors
        (
            id  INTEGER PRIMARY KEY,
            red INTEGER NOT NULL,
            green INTEGER NOT NULL,
            blue INTEGER NOT NULL,
            alpha REAL NOT NULL,
            hex TEXT NOT NULL
        );", params![]).unwrap();

        connection.execute("CREATE TABLE IF NOT EXISTS
        Palettes
        (
            id  INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );", params![]).unwrap();

        connection.execute("CREATE TABLE IF NOT EXISTS
        Palette_Color_Junction
        (
            id  INTEGER PRIMARY KEY,
            palette_id INTEGER NOT NULL,
            color_id INTEGER NOT NULL,
            FOREIGN KEY (palette_id) REFERENCES Palettes(id),
            FOREIGN KEY (color_id) REFERENCES Colors(id)
        );", params![]).unwrap();

        Ok(())
    }

    // ######
    // # ADDING VALUES TO TABLES
    // ######

    fn add_palette(&self, tx: &Transaction, name: String) -> Result<i64, Box<dyn Error>> {
        let mut stmt = tx.prepare("INSERT INTO Palettes (name) VALUES ( ? );")?;
        stmt.execute(params![
            name,
        ])?;
        Ok(tx.last_insert_rowid())
    }

    fn add_color(&self, tx: &Transaction, hex: String, r: i64, g: i64, b: i64, a: f64) -> Result<i64, Box<dyn Error>> {        
        let mut stmt = tx.prepare(format!("SELECT id FROM Colors WHERE red={} AND green={} AND blue={} AND alpha={} AND hex=\"{}\";", r, g, b, a, hex).as_str())?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            Ok(id)
        })?;

        let ids: Vec<i64> = rows.filter_map(|r| r.ok()).collect();
        if !ids.is_empty() {
            return Ok(ids[0]);
        }
        
        let mut stmt = tx.prepare("INSERT INTO Colors (red, green, blue, alpha, hex) VALUES  ( ?, ?, ?, ?, ? );")?;
        stmt.execute(params![
            r, g, b, a, hex,
        ])?;
        Ok(tx.last_insert_rowid())
    }

    fn add_pc_junction(&self, tx: &Transaction, palette_id: i64, color_id: i64) -> Result<i64, Box<dyn Error>> {        
        let mut stmt = tx.prepare(format!("SELECT id FROM Palette_Color_Junction WHERE palette_id={} AND color_id={};", palette_id, color_id).as_str())?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            Ok(id)
        })?;

        let ids: Vec<i64> = rows.filter_map(|r| r.ok()).collect();
        if !ids.is_empty() {
            return Ok(ids[0]);
        }
        
        let mut stmt = tx.prepare("INSERT INTO Palette_Color_Junction (palette_id, color_id) VALUES  ( ?, ? );")?;
        stmt.execute(params![
            palette_id, color_id,
        ])?;
        Ok(tx.last_insert_rowid())
    }

    pub fn add_palette_from_extracted(&self, palette_name: String, colors: &Vec<ExtractedColor>) -> bool {
        match self.add_palette_from_extracted_(palette_name, colors) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }

    }

    fn add_palette_from_extracted_(&self, palette_name: String, colors: &Vec<ExtractedColor>) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: add_palette_from_extracted")?;
        let tx = conn.transaction()?;

        match self.add_palette(&tx, palette_name) {
            Ok(palette_id) => {
                for color in colors {
                    let rgba = color.rgba_tuple();
                    match self.add_color(&tx, color.hex_name(), rgba.0 as i64, rgba.1 as i64, rgba.2 as i64, rgba.3 as f64) {
                        Ok(color_id) => {
                            match self.add_pc_junction(&tx, palette_id, color_id) {
                                Ok(_junction_id) => {
                                },
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                        },
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
                tx.commit()?;                
                Ok(())
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub fn add_palette_new(&self, palette_name: String, hex: String, rgba: (i64, i64, i64, f64)) -> bool {
        match self.add_palette_new_(palette_name, hex, rgba) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }

    }

    fn add_palette_new_(&self, palette_name: String, hex: String, rgba: (i64, i64, i64, f64)) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: add_palette_new")?;
        let tx = conn.transaction()?;

        match self.add_palette(&tx, palette_name) {
            Ok(palette_id) => {
                match self.add_color(&tx, hex, rgba.0, rgba.1, rgba.2, rgba.3) {
                    Ok(color_id) => {
                        match self.add_pc_junction(&tx, palette_id, color_id) {
                            Ok(_junction_id) => {
                                tx.commit()?;
                                Ok(())
                            },
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn duplicate_palette(&self, palette_id: i64, duplicate_name: String) -> bool {
        match self.query_colors_by_palette_id(palette_id) {
            Ok(color_ids) => {
                if color_ids.is_empty() {
                    return false;
                }

                match self.duplicate_palette_(duplicate_name, color_ids) {
                    Ok(_) => {
                        self.emit_by_name::<()>("populate-model", &[]);
                        return true
                    },
                    Err(e) => {
                        error!("{}", e);
                        return false
                    },
                }

            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }


    }

    fn duplicate_palette_(&self, duplicate_name: String, color_ids: Vec<i64>) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: duplicate_palette")?;
        let tx = conn.transaction()?;

        match self.add_palette(&tx, duplicate_name) {
            Ok(palette_id) => {
                for color_id in color_ids {
                    match self.add_pc_junction(&tx, palette_id, color_id) {
                        Ok(_junction_id) => {},
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
            },
            Err(e) => return Err(e),
        }

        tx.commit()?;
        Ok(())
    }

    pub fn add_color_to_palette(&self, palette_id: i64, hex: String, rgba: (i64, i64, i64, f64)) -> bool {
        match self.add_color_to_palette_(palette_id, hex, rgba) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }
    }

    fn add_color_to_palette_(&self, palette_id: i64, hex: String, rgba: (i64, i64, i64, f64)) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: add_color_to_palette_")?;
        let tx = conn.transaction()?;

        match self.add_color(&tx, hex, rgba.0, rgba.1, rgba.2, rgba.3) {
            Ok(color_id) => {
                match self.add_pc_junction(&tx, palette_id, color_id) {
                    Ok(_junction_id) => {
                        tx.commit()?;
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }

    // ######
    // # MODIFY VALUES
    // ######
    
    pub fn rename_palette(&self, palette_id: i64, new_name: String) -> bool {
        match self.rename_palette_(palette_id, new_name) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }
    }
    
    fn rename_palette_(&self, palette_id: i64, new_name: String) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: rename_palette")?;
        let tx = conn.transaction()?;

        tx.execute(format!("UPDATE Palettes SET name = \"{}\" WHERE id = {};", new_name, palette_id).as_str(), ())?;

        match self.prune_colors(&tx) {
            Ok(_) => {
                match self.prune_palletes(&tx) {
                    Ok(_) => {
                        tx.commit()?;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
    

    pub fn remove_color_from_palette(&self, color_id: i64, palette_id: i64) -> bool {
        match self.remove_color_from_palette_(color_id, palette_id) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }
    }

    fn remove_color_from_palette_(&self, color_id: i64, palette_id: i64) -> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: remove_color_from_palette")?;
        let tx = conn.transaction()?;

        tx.execute(format!("DELETE FROM Palette_Color_Junction WHERE color_id = {} AND palette_id = {};", color_id, palette_id).as_str(), ())?;

        match self.prune_colors(&tx) {
            Ok(_) => {
                match self.prune_palletes(&tx) {
                    Ok(_) => {
                        tx.commit()?;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    // ######
    // # REMOVE VALUES 
    // ######
    
    pub fn delete_palette(&self, palette_id: i64) -> bool {
        match self.delete_palette_(palette_id) {
            Ok(_) => {
                self.emit_by_name::<()>("populate-model", &[]);
                return true
            },
            Err(e) => {
                error!("{}", e);
                return false
            },
        }
    }

    fn delete_palette_(&self, palette_id: i64)-> Result<(), Box<dyn Error>> {
        let mut conn = self.imp().conn.borrow_mut();
        let conn = conn.as_mut().ok_or("Connection not established: delete_palette")?;
        let tx = conn.transaction()?;

        tx.execute(format!("DELETE FROM Palette_Color_Junction WHERE palette_id = {};", palette_id).as_str(), ())?;
        tx.execute(format!("DELETE FROM Palettes WHERE id = {};", palette_id).as_str(), ())?;

        match self.prune_colors(&tx) {
            Ok(_) => {
                tx.commit()?;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }



    fn prune_colors(&self, tx: &Transaction) -> Result<(), Box<dyn Error>> {
        let mut stmt = tx.prepare("SELECT id FROM Colors;")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            Ok(id)
        })?;
        for row in rows {
            let color_id = row?;

            let mut stmt = tx.prepare(format!("SELECT id FROM Palette_Color_Junction WHERE color_id = {};", color_id).as_str())?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(id)
            })?;

            let mut junctions = Vec::new();
            for row in rows {
                junctions.push(row?);
            }

            if junctions.is_empty() {
                tx.execute(format!("DELETE FROM Colors WHERE id = {};", color_id).as_str(), ())?;
            }

        }

        Ok(())
    }


    fn prune_palletes(&self, tx: &Transaction) -> Result<(), Box<dyn Error>> {
        let mut stmt = tx.prepare("SELECT id FROM Palettes;")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            Ok(id)
        })?;
        for row in rows {
            let palette_id = row?;

            let mut stmt = tx.prepare(format!("SELECT id FROM Palette_Color_Junction WHERE palette_id = {};", palette_id).as_str())?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(id)
            })?;

            let mut junctions = Vec::new();
            for row in rows {
                junctions.push(row?);
            }

            if junctions.is_empty() {
                tx.execute(format!("DELETE FROM Palettes WHERE id = {};", palette_id).as_str(), ())?;
            }

        }

        Ok(())
    }

    // ######
    // # QUERY VALUES
    // ######

    pub fn query_colors(&self) -> Result<Vec<(i64, i64, i64, i64, f64, String)>, Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let conn = conn.as_ref().ok_or("Connection not established")?;
        
        let mut stmt = conn.prepare("SELECT * FROM Colors;")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let r: i64 = row.get(1)?;
            let g: i64 = row.get(2)?;
            let b: i64 = row.get(3)?;
            let a: f64 = row.get(4)?;
            let hex: String = row.get(5)?;
            Ok((id, r, g, b, a, hex))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    pub fn query_palettes(&self) -> Result<Vec<(i64, String)>, Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let conn = conn.as_ref().ok_or("Connection not established")?;
        
        let mut stmt = conn.prepare("SELECT * FROM Palettes;")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((id, name))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    pub fn query_palette_color_junction(&self) -> Result<Vec<(i64, i64, i64)>, Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let conn = conn.as_ref().ok_or("Connection not established")?;
        
        let mut stmt = conn.prepare("SELECT * FROM Palette_Color_Junction;")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let palette_id: i64 = row.get(1)?;
            let color_id: i64 = row.get(2)?;
            Ok((id, palette_id, color_id))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    pub fn query_n_palettes(&self) -> i64 {
        match self.query_n_palettes_() {
            Ok(n) => return n,
            Err(_) => 0,
        }
    }

    fn query_n_palettes_(&self) -> Result<i64, Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let conn = conn.as_ref().ok_or("Connection not established")?;

        let mut stmt = conn.prepare("SELECT count( * ) FROM Palettes;")?;
        let count = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    pub fn query_colors_by_palette_id(&self, palette_id: i64) -> Result<Vec<i64>, Box<dyn Error>> {
        let conn = self.imp().conn.borrow();
        let conn = conn.as_ref().ok_or("Connection not established")?;

        let mut stmt = conn.prepare("SELECT color_id FROM Palette_Color_Junction WHERE palette_id = ?;")?;
        let rows = stmt.query_map([palette_id], |row| row.get(0))?;
        let colors = rows.filter_map(|r| r.ok()).collect();
        Ok(colors)
    }
}
