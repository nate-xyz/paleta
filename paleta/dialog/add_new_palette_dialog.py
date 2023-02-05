from gi.repository import Adw, Gtk

from paleta.model import Color
from paleta.util import rgb_to_hex
from .simple_row import SimplePaletteRow
from paleta.pages import ColorSquare

import re

@Gtk.Template(resource_path='/io/nxyz/Paleta/add_new_palette_dialog.ui')
class AddNewPaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'AddNewPaletteDialog'

    adw_entry_row = Gtk.Template.Child(name="adw_entry_row")
    color_selection_row = Gtk.Template.Child(name="color_selection_row")
    picker_button = Gtk.Template.Child(name="picker_button")
    currently_selected_label = Gtk.Template.Child(name="currently_selected_label")
    currently_selected_color_square = Gtk.Template.Child(name="currently_selected_color_square")
    revealer = Gtk.Template.Child(name="revealer")
    color_instruction_label = Gtk.Template.Child(name="color_instruction_label")

    name = "Palette"

    def __init__(self, window, database, model) -> None:
        super().__init__()
        self.window = window
        self.db = database
        self.model = model
        self.color = None
        self.set_transient_for(self.window)
        self.set_name("Palette #{}".format(self.db.query_n_palettes()+1))
        self.dialog = Gtk.ColorChooserDialog.new('Choose color to add to new palette', self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

        self.picker_button.connect('clicked', lambda _button: self.dialog.show())
    
        if len(model.get_colors().items()) > 0:
            self.color_selection_row.set_child(SimplePaletteRow(self.model, self.set_current_color))
        else:
            self.color_instruction_label.set_label("Pick a color to add to new palette.")

    def set_name(self, name):
        self.name = name 
        self.adw_entry_row.set_text(self.name)

    def set_current_color(self, color: Color):
        self.revealer.set_reveal_child(False)
        self.currently_selected_label.set_label("Currently selected color: {}".format(color.hex))
        self.currently_selected_color_square.set_child(ColorSquare(110, color.rgb_name))
        self.color = color
        if not self.revealer.get_reveal_child():
            self.revealer.set_reveal_child(True)
            
    def init_chooser(self):
        self.dialog = Gtk.ColorChooserDialog.new('Choose color to add to new palette', self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

    def chooser_response(self, dialog, response):
        print(response)
        if response == Gtk.ResponseType.OK:
            color = dialog.get_rgba()
            rgb_name = color.to_string()
            rgba = [int(i) for i in re.search('\(([^)]+)', rgb_name).group(1).split(',')]            
            hex = "#{}".format(rgb_to_hex(*rgba))
            if len(rgba) <= 3:
                r, g, b = rgba 
                a = 1.0 
            else:
                r, g, b, a = rgba
            self.set_current_color(Color(None, r, g, b, a, hex))

        dialog.close()
        self.init_chooser()

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'add':
            if self.color == None:
                self.window.add_toast("Unable to add palette, must select a color.")
                return 
            
            name = self.adw_entry_row.get_text()
            if name == '':
                name = self.name

            if self.db.add_palette_new(name, self.color.hex, *self.color.rgba):
                self.window.add_toast("Created new palette «{}»".format(name))
            else:
                self.window.add_toast("Unable to add new palette «{}»".format(name))






