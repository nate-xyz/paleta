from gi.repository import Gio, Gtk

from paleta.model import Palette, Color
from .palette_color_card import PaletteColorCard


@Gtk.Template(resource_path='/io/nxyz/Paleta/palette_row.ui')
class PaletteRow(Gtk.ListBoxRow):
    __gtype_name__ = 'PaletteRow'

    title_label = Gtk.Template.Child(name="title_label")
    flow_box = Gtk.Template.Child(name="flow_box")

    def __init__(self, palette: Palette, copy_callback) -> None:
        super().__init__()
        self.palette = palette
        self.copy_callback = copy_callback
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

    def update_edit_view(self, edit_mode):
        if edit_mode:
            pass 
        else:
            pass

    def flowbox_factory(self, color):
        return PaletteColorCard(color, self.copy_callback)


