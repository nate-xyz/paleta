from gi.repository import Adw, GLib, Gio, Gtk

import os
from multiprocessing import Pipe, Process
from colorthief import ColorThief

from .dropped_image import DroppedImage
from .extracted_color_row import ExtractedColorRow, ExtractedColor
from paleta.dialog import SavePaletteDialog

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/color_thief_panel.ui')
class ColorThiefPanel(Adw.Bin):
    __gtype_name__ = 'ColorThiefPanel'

    image_bin = Gtk.Template.Child(name="image_bin")
    count_amount_spin = Gtk.Template.Child(name="count_amount_spin")
    quality_dropdown = Gtk.Template.Child(name="quality_dropdown")
    palette_box = Gtk.Template.Child(name="palette_box")
    spinner = Gtk.Template.Child(name="spinner")
    colors_list_box = Gtk.Template.Child(name="colors_list_box")
    save_button = Gtk.Template.Child(name="save_button")

    def __init__(self) -> None:
        super().__init__()
        self.image = None

        self.list_store = Gio.ListStore(item_type=ExtractedColor)
        self.colors_list_box.bind_model(self.list_store, self.listbox_factory)
        self.save_button.connect('clicked', lambda _button: self.save_palette())
        self.colors_list_box.connect('row_selected', lambda _listbox, row: self.window.copy_color(row.hex_name))

        self.quality_dropdown.set_selected(1)

        self.count_amount = self.count_amount_spin.get_value()
        self.quality = self.get_quality()

        self.count_amount_spin.connect('value-changed', self.on_spin_value_change)
        self.quality_dropdown.connect('notify::selected', self.on_quality_value_change)

        self.parent_conn, self.child_conn = Pipe(duplex=False)

        self.process_id = None
        self.processes = []

        GLib.io_add_watch(self.parent_conn.fileno(), GLib.IO_IN, self.extraction_done)

    def get_quality(self) -> int:
        match self.quality_dropdown.get_selected():
            case 0:
                return 1
            case 1:
                return 3
            case 2:
                return 10

    def set_image(self, image: DroppedImage):
        self.set_visible(True)
        self.image_bin.set_child(None)
        self.image_bin.set_child(image)
        self.image = image
        self.list_store.remove_all()
        self.start_extraction()

    def saturate(self, window, database):
        self.window = window
        self.db = database

    def start_extraction(self):
        if self.image != None:
            self.window.go_to_image_drop_page()

            self.palette_box.set_visible(False)
            self.spinner.set_visible(True)
            self.spinner.start()

            for p in self.processes:
                p.terminate()

            self.processes = []

            try:
                process = Process(
                    target=color_extraction,
                    args=(
                        self.child_conn,
                        self.image.image_path,
                        int(self.count_amount),
                        int(self.quality),
                    ))
                process.daemon = True 
                process.start()
                self.process_id = process.ident
                self.processes.append(process)
            except:
                self.window.add_error_toast(_("Unable to start palette extraction."))
                
        else:
            self.window.add_error_toast(_("Unable to start palette extraction, no image loaded."))

    def extraction_done(self, source, condition) -> bool:
        assert self.parent_conn.poll()
        try:
            colors, process_id = self.parent_conn.recv()
        except EOFError:
            return True

        if self.process_id != process_id:
            return True

        self.spinner.stop()
        self.spinner.set_visible(False)
        self.palette_box.set_visible(True)

        self.list_store.remove_all()
        colors_n = len(colors)
        if colors_n == 0:
            self.window.add_error_toast(_("Unable to extract colors from image."))
            return

        for rgb in colors:
            self.list_store.append(ExtractedColor(rgb))

        return True

    def save_palette(self):
        if self.image == None:
            self.window.add_error_toast(
                _("Unable to save palette, no image loaded."))
            return

        if len(self.list_store) == 0:
            self.window.add_error_toast(
                _("Unable to save palette, no colors extracted."))
            return

        SavePaletteDialog([ec for ec in self.list_store],
                          self.window, self.db).show()

    def listbox_factory(self, color):
        return ExtractedColorRow(color)

    def on_spin_value_change(self, spin_button):
        self.count_amount = spin_button.get_value()
        self.start_extraction()

    def on_quality_value_change(self, dropdown, value):
        self.quality = self.get_quality()
        self.start_extraction()


def color_extraction(pipe_connection, uri, count, quality):
    color_thief = ColorThief(uri)
    colors = color_thief.get_palette(color_count=count, quality=quality)[:count]
    pipe_connection.send((colors, os.getpid()))
