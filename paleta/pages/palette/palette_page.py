from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango

from .palette_row import PaletteRow
from paleta.model import Palette

@Gtk.Template(resource_path='/io/nxyz/Paleta/palette_page.ui')
class PalettePage(Adw.Bin):
    __gtype_name__ = 'PalettePage'

    list_box = Gtk.Template.Child(name="list_box")

    def __init__(self) -> None:
        super().__init__()
        self.list_store = Gio.ListStore(item_type=Palette)
        self.list_box.bind_model(self.list_store, self.listbox_factory)
        self.copy_callback = None

    def update_view(self, model=None):
        self.list_store.remove_all()
        for _, palette in self.model.get_palettes().items():
            self.list_store.append(palette)

    def listbox_factory(self, palette):
        return PaletteRow(palette, self.copy_callback)

    def set_model(self, model):
        self.model = model
        self.model.connect('populated', self.update_view)