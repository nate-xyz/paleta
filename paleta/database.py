from gi.repository import GObject, Adw, GLib

import os
import sys 
import traceback
import time
import logging
import datetime
import sqlite3
from collections import defaultdict
import pprint
pp = pprint.PrettyPrinter(indent=4)
from itertools import tee 
import threading 

from paleta.model import Model

from xdg.BaseDirectory import xdg_data_home

APP_PREFIX = "io.nxyz.Paleta"
APP_DATA_DIR =  os.path.join(xdg_data_home, APP_PREFIX)
sqlite3.enable_callback_tracebacks(True)

class Database(GObject.GObject):
    __gtype_name__ = 'Database'

    # __gsignals__ = {
    # }

    database_name = "paleta_database.db"

    def __init__(self) -> None:
        super().__init__()
        
        self.window = None
        self.model = None
        self.database_path = os.path.join(APP_DATA_DIR, self.database_name)
        
        print(self.database_path)

    def try_loading_database(self) -> bool:
        print('Database try_loading_database', threading.get_ident())
                 
        if self.open_connection_to_db():
            GLib.idle_add(self.model.populate)  # generate internal model
            if self.window != None:
                pass #TODO: refresh palette page
                #GLib.idle_add(self.window.)

            print('Database loaded db!', threading.get_ident())
            return True
        else:
            return False
          
    #opens connection to database, return whether the database is newly created or not
    def open_connection_to_db(self) -> bool:
        print('open_connection_to_db')
        try:
            self.con = sqlite3.connect(
                "file:{}?mode=rw".format(self.database_path), 
                uri=True, 
                check_same_thread=False, 
                detect_types=sqlite3.PARSE_DECLTYPES | sqlite3.PARSE_COLNAMES)

            print('open database by existing uri')
        
        except:
            try:
                os.mkdir(APP_DATA_DIR)
                self.con = sqlite3.connect(
                    self.database_path, 
                    check_same_thread=False, 
                    detect_types=sqlite3.PARSE_DECLTYPES | sqlite3.PARSE_COLNAMES)
                print('open NEW database')
            except Exception as e:
                print(e)
                return False
        
        self.cur = self.con.cursor()

        self.setup_db_tables()  # setup tables

        # print location of database
        for id_, name, filename in self.con.execute('PRAGMA database_list'):
            print('database location', filename)

        return True

    def close(self):
        self.con.close()

    def remove_db(self):
        if os.path.exists(self.database_name):
            os.remove(self.database_name)

    def setup_db_tables(self):
        self.cur.execute("PRAGMA foreign_keys = ON")

        # make genre table
        self.cur.execute("""CREATE TABLE IF NOT EXISTS
        Colors
        (
            id  INTEGER PRIMARY KEY,
            red INTEGER NOT NULL,
            green INTEGER NOT NULL,
            blue INTEGER NOT NULL,
            alpha REAL NOT NULL,
            hex TEXT NOT NULL
        );""")

        self.cur.execute("""CREATE TABLE IF NOT EXISTS
        Palettes
        (
            id  INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );""")


        self.cur.execute("""CREATE TABLE IF NOT EXISTS
        Palette_Color_Junction
        (
            id  INTEGER PRIMARY KEY,
            palette_id INTEGER NOT NULL,
            color_id INTEGER NOT NULL,
            FOREIGN KEY (palette_id) REFERENCES Palettes(id),
            FOREIGN KEY (color_id) REFERENCES Colors(id)
        );""")

        self.con.commit()


######
# ADDING VALUES TO TABLES
######

    def add_palette(self, name: str):
        self.cur.execute("""
        INSERT INTO Palettes (name) 
        VALUES ( ? );""", (name, ) )
        return self.cur.lastrowid 

    def add_color(self, hex, r, g, b, a=1.0):
        self.cur.execute("""SELECT id FROM Colors WHERE 
        red={} AND green={} AND blue={} AND alpha={} AND hex=\"{}\";""".format(r, g, b, a, hex))
        ids = self.cur.fetchall()
        if ids != []:
            return ids[0][0]
        
        self.cur.execute("""
        INSERT INTO Colors (red, green, blue, alpha, hex) 
        VALUES ( ?, ?, ?, ?, ? );""", 
        (r, g, b, a, hex, ) )
        return self.cur.lastrowid 

    def add_pc_junction(self, palette_id, color_id):
        self.cur.execute("""SELECT id FROM Palette_Color_Junction WHERE 
        palette_id={} AND color_id={};""".format(palette_id, color_id))
        ids = self.cur.fetchall()
        if ids != []:
            return ids[0][0]

        self.cur.execute("""
        INSERT INTO Palette_Color_Junction (palette_id, color_id) VALUES ( ?, ? );""", 
        (palette_id, color_id) )
        return self.cur.lastrowid 

    #add palette from drop page
    def add_palette_from_extracted(self, name: str, colors):
        try:
            palette_id = self.add_palette(name)
            color_ids = [self.add_color(c.hex_name, *c.rgb) for c in colors]
            [self.add_pc_junction(palette_id, cid) for cid in color_ids]

            self.con.commit()
            self.model.populate()
            
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False

    def duplicate_palette(self, palette_id, duplicate_name):
        try:
            color_ids = self.query_colors_by_palette_id(palette_id)
            if color_ids == []:
                return False
            
            palette_id = self.add_palette(duplicate_name)
            [self.add_pc_junction(palette_id, cid) for cid in color_ids]

            self.con.commit()
            self.model.populate()

            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False

    def add_color_to_palette(self, palette_id, hex, r, g, b, a=1.0):
        try:
            color_id = self.add_color(hex, r, g, b, a=1.0)
            self.add_pc_junction(palette_id, color_id)

            self.con.commit()
            self.model.populate()
            
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False

######
# MODIFY VALUES 
######

    def rename_palette(self, palette_id, new_name) -> bool:
        try:
            self.cur.execute("""
            UPDATE Palettes 
            SET name = \"{}\"
            WHERE id = {};""".format(new_name, palette_id))
            self.con.commit()
            self.model.populate()
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False

    def remove_color_from_palette(self, color_id, palette_id) -> bool:
        try:
            self.cur.execute("DELETE FROM Palette_Color_Junction WHERE color_id = {} AND palette_id = {};".format(color_id, palette_id))

            self.prune_colors()
            self.prune_palletes()

            self.con.commit()
            self.model.populate()
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False

######
# REMOVE VALUES 
######

    def delete_color(self, color_id, commit=True):
        try:
            self.cur.execute("DELETE FROM Palette_Color_Junction WHERE color_id = {};".format(color_id))
            self.cur.execute("DELETE FROM Colors WHERE id = {};".format(color_id))
            
            self.prune_palletes()
            
            if commit:
                self.con.commit()
            
            self.model.populate()
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False
    
    def delete_palette(self, palette_id, commit=True):
        try:
            self.cur.execute("DELETE FROM Palette_Color_Junction WHERE palette_id = {};".format(palette_id))
            self.cur.execute("DELETE FROM Palettes WHERE id = {};".format(palette_id))
            self.prune_colors()
            
            if commit:
                self.con.commit()
            
            self.model.populate()
            return True
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return False


    def prune_colors(self):
        self.cur.execute("SELECT id FROM Colors;")
        for color_id in [color_id for tuple in self.cur.fetchall() for color_id in tuple]:
            self.cur.execute("SELECT id FROM Palette_Color_Junction WHERE color_id = {};".format(color_id))
            if self.cur.fetchall() == []:
                print("Deleting color id", color_id)
                self.cur.execute("DELETE FROM Colors WHERE id = {};".format(color_id))

    def prune_palletes(self):
        self.cur.execute("SELECT id FROM Palettes;")
        for palette_id in [palette_id for tuple in self.cur.fetchall() for palette_id in tuple]:
            self.cur.execute("SELECT id FROM Palette_Color_Junction WHERE palette_id = {};".format(palette_id))
            if self.cur.fetchall() == []:
                print("Deleting palette id", palette_id)
                self.cur.execute("DELETE FROM Palettes WHERE id = {};".format(palette_id))


######
# QUERY VALUES
######

    def query_colors(self):
        self.cur.execute("SELECT * FROM Colors;")
        return self.cur.fetchall()

    def query_palettes(self):
        self.cur.execute("SELECT * FROM Palettes;")
        return self.cur.fetchall()

    def query_palette_color_junction(self):
        self.cur.execute("SELECT * FROM Palette_Color_Junction;")
        return self.cur.fetchall()

    def query_n_palettes(self):
        self.cur.execute("SELECT count( * ) FROM palettes;")
        return self.cur.fetchall()[0][0]

    def query_colors_by_palette_id(self, palette_id):
        colors = self.cur.execute("SELECT color_id FROM Palette_Color_Junction WHERE palette_id={};".format(palette_id))
        return [color_id for tuple in colors for color_id in tuple]

