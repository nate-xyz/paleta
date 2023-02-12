from gi.repository import Adw, Gio, Gtk

from .palette_row import PaletteRow
from paleta.model import Model, Palette
from paleta.dialog import AddNewPaletteDialog


@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/palette_page.ui')
class PalettePage(Adw.Bin):
    __gtype_name__ = 'PalettePage'

    list_box = Gtk.Template.Child(name="list_box")
    status = Gtk.Template.Child(name="status")
    add_palette_button = Gtk.Template.Child(name="add_palette_button")

    def __init__(self) -> None:
        super().__init__()
        self.list_store = Gio.ListStore(item_type=Palette)
        self.list_box.bind_model(self.list_store, self.listbox_factory)
        self.edit_mode = False
        self.window = None
        self.database = None
        self.add_palette_button.connect('clicked', lambda _button: self.show_new_palette_dialog())
        
    def saturate(self, window, database, model: Model):
        self.window = window 
        self.edit_button = window.edit_palette_button
        self.database = database
        self.model = model
        self.model.connect('populated', self.update_view)

    def update_view(self, model=None):
        self.list_store.remove_all()
        palettes = self.model.get_palettes().items()
        if len(palettes) == 0:
           self.status.show()
           self.list_box.hide()
           self.edit_button.set_css_classes(['flat'])
           self.edit_button.hide()
           self.edit_mode = False
        else:
            self.status.hide()
            self.list_box.show()
        for _, palette in palettes:
            self.list_store.append(palette)
        self.update_edit_view()

    def update_edit_view(self):
        for pc in self.list_box:
            pc.edit_mode = self.edit_mode
            pc.update_edit_view()

    def listbox_factory(self, palette):
        return PaletteRow(palette, self.window, self.database, self.model)

    def toggle_edit_mode(self):
        if len(self.list_store) > 0:
            self.set_edit_mode(not self.edit_mode)
            self.window.go_to_palette_page()
        else:
            self.window.add_error_toast(_("Cannot toggle edit mode, no palettes added."))

    def set_edit_mode(self, mode):
        self.edit_mode = mode
        if self.edit_mode:
            self.edit_button.set_css_classes(['opaque', 'edit-action-button'])
        else:
            self.edit_button.set_css_classes(['flat'])
        self.update_edit_view()

    def show_new_palette_dialog(self):
        def after_dialog():
            self.set_edit_mode(False)
            self.edit_button.show()
            self.window.go_to_palette_page()

        dialog = AddNewPaletteDialog(self.window, self.database, self.model)
        dialog.connect('response', lambda dialog, response: after_dialog())
        dialog.show()