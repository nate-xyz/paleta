from gi.repository import Gio, Gtk

from .palette_color_card import PaletteColorCard
from paleta.model import Palette, Color
from paleta.dialog import RenamePaletteDialog, DuplicatePaletteDialog, DeletePaletteDialog, AddColorDialog

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/palette_row.ui')
class PaletteRow(Gtk.ListBoxRow):
    __gtype_name__ = 'PaletteRow'

    title_label = Gtk.Template.Child(name="title_label")
    flow_box = Gtk.Template.Child(name="flow_box")
    edit_mode_revealer = Gtk.Template.Child(name="edit_mode_revealer")

    edit_name_button = Gtk.Template.Child(name="edit_name_button")
    duplicate_palette_button = Gtk.Template.Child(name="duplicate_palette_button")
    add_color_button = Gtk.Template.Child(name="add_color_button")
    delete_palette_button = Gtk.Template.Child(name="delete_palette_button")

    def __init__(self, palette: Palette, window, database, model) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.db = database
        self.model = model

        self.title_label.set_label(palette.name)
        self.list_store = Gio.ListStore(item_type=Color)
        self.flow_box.bind_model(self.list_store, self.flowbox_factory)
        self.flow_box.set_min_children_per_line(len(self.palette.colors))
        self.flow_box.set_max_children_per_line(len(self.palette.colors))

        self.edit_name_button.connect('clicked', self.show_dialog)
        self.duplicate_palette_button.connect('clicked', self.show_dialog)
        self.delete_palette_button.connect('clicked', self.show_dialog)
        self.add_color_button.connect('clicked', self.show_dialog)

        self.edit_mode = False
        self.update_view()

    def show_dialog(self, button):
        match button:
            case self.edit_name_button:
                dialog = RenamePaletteDialog(self.palette, self.window, self.db)
            case self.duplicate_palette_button:
                dialog = DuplicatePaletteDialog(self.palette, self.window, self.db)
            case self.delete_palette_button:
                dialog = DeletePaletteDialog(self.palette, self.window, self.db)
            case self.add_color_button:
                dialog = AddColorDialog(self.palette, self.window, self.db, self.model)
        dialog.connect('response', lambda dialog, response: self.window.palette_page.set_edit_mode(False))
        dialog.show()

    def update_view(self):
        self.list_store.remove_all()
        for color in self.palette.colors:
            self.list_store.append(color)
        self.update_edit_view()
    
    def update_edit_view(self):
        self.edit_mode_revealer.set_reveal_child(self.edit_mode)
        for pcc in self.flow_box:
            pcc.edit_mode = self.edit_mode
            pcc.update_edit_view()

    def flowbox_factory(self, color):
        return PaletteColorCard(color, self.palette, self.window, self.db)

