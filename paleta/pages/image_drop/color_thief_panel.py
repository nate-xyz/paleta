from gi.repository import Adw, GLib, Gio, Gtk

from threading import Thread
from colorthief import ColorThief

from .dropped_image import DroppedImage
from .extracted_color_row import ExtractedColorRow, ExtractedColor
from paleta.dialog import SavePaletteDialog

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/color_thief_panel.ui')
class ColorThiefPanel(Adw.Bin):
    __gtype_name__ = 'ColorThiefPanel'

    image_bin = Gtk.Template.Child(name="image_bin")
    extract_button = Gtk.Template.Child(name="extract_button")
    count_amount_spin = Gtk.Template.Child(name="count_amount_spin")
    quality_spin = Gtk.Template.Child(name="quality_spin")
    palette_box = Gtk.Template.Child(name="palette_box")
    spinner = Gtk.Template.Child(name="spinner")
    colors_list_box = Gtk.Template.Child(name="colors_list_box")
    save_button = Gtk.Template.Child(name="save_button")
    
    def __init__(self) -> None:
        super().__init__()
        self.image = None

        self.list_store = Gio.ListStore(item_type=ExtractedColor)
        self.colors_list_box.bind_model(self.list_store, self.listbox_factory)

        self.extract_button.connect('clicked', lambda _button: self.start_extraction())
        self.save_button.connect('clicked', lambda _button: self.save_palette())
        self.colors_list_box.connect('row_selected', lambda _listbox, row: self.window.copy_color(row.hex_name))

    def set_image(self, image: DroppedImage):
        self.set_visible(True)
        self.image_bin.set_child(None)
        self.image_bin.set_child(image)
        self.image = image
        self.list_store.remove_all()

    def saturate(self, window, database):
        self.window = window 
        self.db = database

    def start_extraction(self):
        if self.image != None:
            self.window.go_to_image_drop_page()
            
            self.palette_box.set_visible(False)
            self.spinner.set_visible(True)
            self.spinner.start()
            self.extract_button.set_sensitive(False)
            thread = Thread(target=color_extraction, args=(
                self.image.image_path, 
                int(self.count_amount_spin.get_value()), 
                int(self.quality_spin.get_value()),
                self.extraction_done,
                ))
            thread.daemon = True
            thread.start()
        else:
            self.window.add_error_toast("Unable to start palette extraction, no image loaded.")
       
    def extraction_done(self, colors):
        self.spinner.stop()
        self.spinner.set_visible(False)
        self.extract_button.set_sensitive(True)
        self.palette_box.set_visible(True)
        
        self.list_store.remove_all()
        colors_n = len(colors)
        if colors_n == 0:
            self.window.add_error_toast("Unable to extract colors from image.")
            return
            
        for rgb in colors:
            self.list_store.append(ExtractedColor(rgb))
        
        self.window.add_toast("Extracted {} colors from {}!".format(colors_n, self.image.image_path))

    def save_palette(self):
        if self.image == None:
            self.window.add_error_toast("Unable to save palette, no image loaded.")
            return 
        
        if len(self.list_store) == 0:
            self.window.add_error_toast("Unable to save palette, no colors extracted.")
            return 
        
        SavePaletteDialog([ec for ec in self.list_store], self.window, self.db).show()

    def listbox_factory(self, color):
        return ExtractedColorRow(color)


def color_extraction(uri, count, quality, callback):
    color_thief = ColorThief(uri)
    colors = color_thief.get_palette(color_count=count, quality=quality)[:count]
    GLib.idle_add(callback, colors)



