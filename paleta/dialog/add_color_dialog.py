from gi.repository import Adw, Gtk

from paleta.model import Palette, Color
from paleta.util import rgb_to_hex
from .simple_row import SimplePaletteRow
from paleta.pages import ColorSquare

import re

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/add_color_dialog.ui')
class AddColorDialog(Adw.MessageDialog):
    __gtype_name__ = 'AddColorDialog'

    color_selection_row = Gtk.Template.Child(name="color_selection_row")
    picker_button = Gtk.Template.Child(name="picker_button")
    currently_selected_label = Gtk.Template.Child(name="currently_selected_label")
    currently_selected_color_square = Gtk.Template.Child(name="currently_selected_color_square")
    revealer = Gtk.Template.Child(name="revealer")
    color_instruction_label = Gtk.Template.Child(name="color_instruction_label")

    def __init__(self, palette: Palette, window, database, model) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.db = database
        self.model = model
        self.color = None
        self.set_transient_for(self.window)
        self.set_heading(_("Add Color to {}".format(palette.name)))
        # Translators: Do not replace {}
        self.dialog = Gtk.ColorChooserDialog.new(_("Choose new color to add to {}".format(palette.name)), self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

        self.picker_button.connect('clicked', lambda _button: self.dialog.show())
    
        if len(model.get_colors().items()) > 0:
            self.color_selection_row.set_child(SimplePaletteRow(self.model, self.set_current_color))
        else:
            # Translators: Do not replace {}
            self.color_instruction_label.set_label(_("Pick a new color to add to {}.".format(palette.name)))

    def set_current_color(self, color: Color):
        self.revealer.set_reveal_child(False)
        # Translators: Do not replace {}
        self.currently_selected_label.set_label(_("Currently selected color: {}".format(color.hex)))
        self.currently_selected_color_square.set_child(ColorSquare(110, color.rgb_name))
        self.color = color
        if not self.revealer.get_reveal_child():
            self.revealer.set_reveal_child(True)
            
    def init_chooser(self):
        # Translators: Do not replace {}
        self.dialog = Gtk.ColorChooserDialog.new(_("Choose new color to add to {}".format(self.palette.name)), self)
        self.dialog.set_transient_for(self)
        self.dialog.connect('response', self.chooser_response)
        self.dialog.connect('close', lambda dialog: dialog.close())

    def chooser_response(self, dialog, response):
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
            if self.palette == None or self.color == None:
                self.window.add_error_toast(_("Unable to add color."))
                return 
            
            if self.db.add_color_to_palette(self.palette.id, self.color.hex, *self.color.rgba):
                self.window.add_color_toast(self.color.hex, self.palette.name)
            else:
                # Translators: Do not replace {self.color.hex}
                self.window.add_error_toast(_(f"Unable to add color {self.color.hex}."))




