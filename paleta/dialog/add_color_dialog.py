from gi.repository import Adw, Gtk

from paleta.model import Palette, Color
from paleta.util import rgb_to_hex
from .simple_row import SimplePaletteRow

import re

@Gtk.Template(resource_path='/io/nxyz/Paleta/add_color_dialog.ui')
class AddColorDialog(Adw.MessageDialog):
    __gtype_name__ = 'AddColorDialog'

    color_box = Gtk.Template.Child(name="color_box")
    picker_button = Gtk.Template.Child(name="picker_button")
    currently_selected_label = Gtk.Template.Child(name="currently_selected_label")

    def __init__(self, palette: Palette, window, database, model) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.db = database
        self.model = model
        self.color = None
        self.set_transient_for(self.window)
        self.set_heading("Add Color to {}".format(palette.name))

        self.dialog = Gtk.ColorChooserDialog.new('Choose new color to add to {}'.format(palette.name), self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

        self.picker_button.connect('clicked', lambda _button: self.dialog.show())

        if len(model.get_colors().items()) > 0:
            self.color_box.prepend(SimplePaletteRow(self.model, self.set_current_color))

    def set_current_color(self, color: Color):
        self.currently_selected_label.set_label("Currently selected color: {}".format(color.hex))
        self.color = color

    def init_chooser(self):
        self.dialog = Gtk.ColorChooserDialog.new('Choose new color to add to {}'.format(self.palette.name), self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

    def chooser_response(self, dialog, response):
        print(response)
        if response == Gtk.ResponseType.OK:
            color = dialog.get_rgba()
            rgb_name = color.to_string()
            rgba = [int(i) for i in re.search('\(([^)]+)', rgb_name).group(1).split(',')]
            print(rgba)
            self.add_color(rgba)
            dialog.close()
            self.close()
            return 
        dialog.close()
        self.init_chooser()


    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'add':
            if self.palette == None or self.color == None:
                self.window.add_toast("Unable to add color.")
                return 
            

            if self.db.add_color_to_palette(self.palette.id, self.color.hex, *self.color.rgba):
                self.window.add_toast("Added color {} to palette «{}».".format(self.color.hex, self.palette.name))
            else:
                self.window.add_toast("Unable to add color {}.".format(self.color.hex))


    def add_color(self, rgba):
            hex = "#{}".format(rgb_to_hex(*rgba))
            if self.db.add_color_to_palette(self.palette.id, hex, *rgba):
                self.window.add_toast("Added color {} to palette «{}».".format(hex, self.palette.name))
            else:
                self.window.add_toast("Unable to add color {}.".format(hex))


