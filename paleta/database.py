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

    def add_palette(self, name: str, colors):
        print('add_palette {}'.format(name))
        try:
            self.cur.execute("""
            INSERT INTO Palettes (name) 
            VALUES ( ? );""", (name, ) )
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return

        palette_id = self.cur.lastrowid 

        color_ids = [self.add_color(*c.rgb, c.hex_name) for c in colors]

        for cid in color_ids:
            if cid != None:
                self.add_pc_junction(palette_id, cid)

        self.con.commit()


    def add_color(self, r, g, b, hex):
        print('add_color {}'.format(hex))
        try:
            self.cur.execute("""
            INSERT INTO Colors (red, green, blue, alpha, hex) 
            VALUES ( ?, ?, ?, ?, ? );""", 
            (r, g, b, 1.0, hex, ) )
            return self.cur.lastrowid 

        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return None


    def add_pc_junction(self, palette_id, color_id):
        try:
            self.cur.execute("""
            INSERT INTO Palette_Color_Junction (palette_id, color_id) 
            VALUES ( ?, ? );""", 
            (palette_id, color_id) )
            return self.cur.lastrowid 
        except Exception as e:
            print(e)
            logging.error(e, exc_info=True)
            return None


######
# MODIFY VALUES 
######


######
# REMOVE VALUES 
######

    def delete_color(self, color_id, commit=True):
        self.cur.execute("""DELETE FROM Palette_Color_Junction WHERE color_id = {};""".format(color_id))
        self.cur.execute("""DELETE FROM Colors WHERE id = {};""".format(color_id))
        self.prune_palletes()
        if commit:
            self.con.commit()
        self.model.remove_color(color_id)
        return self.cur.lastrowid 

    def delete_pallete(self, palette_id, commit=True):
        self.cur.execute("""DELETE FROM Palette_Color_Junction WHERE palette_id = {};""".format(palette_id))
        self.cur.execute("""DELETE FROM Palettes WHERE id = {};""".format(palette_id))
        self.prune_colors()
        if commit:
            self.con.commit()
        
        self.prune_colors()
        self.model.remove_palette(palette_id)
        return self.cur.lastrowid 


    def prune_colors(self):
        pass

    def prune_palletes(self):
        pass


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



