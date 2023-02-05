from gi.repository import Adw, GLib, Gio, Gtk

from threading import Thread
from colorthief import ColorThief

from .dropped_image import DroppedImage
from .extracted_color_row import ExtractedColorRow, ExtractedColor
from paleta.dialog import SavePaletteDialog

@Gtk.Template(resource_path='/io/nxyz/Paleta/color_thief_panel.ui')
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

        self.extract_button.connect('clicked', self.start_extraction)
        self.save_button.connect('clicked', lambda _button: SavePaletteDialog([ec for ec in self.list_store], self.window, self.db).show())
        self.colors_list_box.connect('row_selected', lambda _listbox, row: self.window.copy_color(row.hex_name))

    def set_image(self, image: DroppedImage):
        self.set_visible(True)
        self.image_bin.set_child(None)
        self.image_bin.set_child(image)
        self.image = image

    def saturate(self, window, database):
        self.window = window 
        self.db = database

    def start_extraction(self, _button):
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
       
    def extraction_done(self, colors):
        self.list_store.remove_all()
        self.spinner.stop()
        self.spinner.set_visible(False)
        self.extract_button.set_sensitive(True)
        self.palette_box.set_visible(True)

        for rgb in colors:
            self.list_store.append(ExtractedColor(rgb))

    def listbox_factory(self, color):
        return ExtractedColorRow(color)


def color_extraction(uri, count, quality, callback):
    color_thief = ColorThief(uri)
    colors = color_thief.get_palette(color_count=count, quality=quality)[:count]
    GLib.idle_add(callback, colors)



