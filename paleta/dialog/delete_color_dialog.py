from gi.repository import Adw, Gtk

from paleta.model import Palette, Color
from paleta.pages import ColorSquare

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/simpler_delete_color_card.ui')
class SimplerDeleteColorCard(Adw.Bin):
    __gtype_name__ = 'SimplerDeleteColorCard'

    color_bin = Gtk.Template.Child(name="color_bin")
    hex_label = Gtk.Template.Child(name="hex_label")
    rgb_label = Gtk.Template.Child(name="rgb_label")

    def __init__(self, color: Color) -> None:
        super().__init__()
        self.color = color
        self.color_bin.set_child(ColorSquare(110, color.rgb_name))
        self.hex_label.set_label(color.hex)
        self.rgb_label.set_label(color.rgb_name)


@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/delete_color_dialog.ui')
class DeleteColorDialog(Adw.MessageDialog):
    __gtype_name__ = 'DeleteColorDialog'

    color_bin = Gtk.Template.Child(name="color_bin")

    def __init__(self, color: Color, palette: Palette, window, database) -> None:
        super().__init__()
        
        self.palette = palette
        self.color = color
        self.window = window
        self.db = database

        self.set_transient_for(self.window)
        self.set_heading("Remove color {} from {}?".format(color.hex, palette.name))

        self.color_bin.set_child(SimplerDeleteColorCard(color))

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'remove':
            if self.palette == None or self.color == None:
                self.window.add_error_toast("Unable to remove color.")
                return 

            if self.db.remove_color_from_palette(self.color.id, self.palette.id):
                self.window.remove_color_toast(self.color.hex, self.palette.name)
            else:
                self.window.add_error_toast(f"Unable to remove color {self.color.hex}.")

