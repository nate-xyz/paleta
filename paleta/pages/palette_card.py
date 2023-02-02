from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Gst

from paleta.model import Palette, Color
from .color_card import ColorCard


@Gtk.Template(resource_path='/io/nxyz/Paleta/palette_card.ui')
class PaletteCard(Adw.Bin):
    __gtype_name__ = 'PaletteCard'

    title_label = Gtk.Template.Child(name="title_label")
    flow_box = Gtk.Template.Child(name="flow_box")

    def __init__(self, palette: Palette) -> None:
        super().__init__()
        self.palette = palette
        self.title_label.set_label(palette.name)
        self.list_store = Gio.ListStore(item_type=Color)
        self.flow_box.bind_model(self.list_store, self.flowbox_factory)
        self.flow_box.set_min_children_per_line(len(self.palette.colors))
        self.flow_box.set_max_children_per_line(len(self.palette.colors))
        self.update_view()

    def update_view(self):
        self.list_store.remove_all()
        for color in self.palette.colors:
            self.list_store.append(color)

    def flowbox_factory(self, color):
        return ColorCard(color)


