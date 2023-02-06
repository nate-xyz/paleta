from gi.repository import Gtk

from paleta.model import Color
from paleta.dialog import DeleteColorDialog
from paleta.pages import ColorSquare

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/palette_color_card.ui')
class PaletteColorCard(Gtk.FlowBoxChild):
    __gtype_name__ = 'PaletteColorCard'

    color_bin = Gtk.Template.Child(name="color_bin")
    hex_label = Gtk.Template.Child(name="hex_label")
    rgb_label = Gtk.Template.Child(name="rgb_label")
    button = Gtk.Template.Child(name="button")
    revealer = Gtk.Template.Child(name="revealer")
    delete_color_button = Gtk.Template.Child(name="delete_color_button")

    def __init__(self, color: Color, palette, window, database) -> None:
        super().__init__()
        self.color = color
        self.palette = palette
        self.window = window
        self.db = database
        self.color_bin.set_child(ColorSquare(110, color.rgb_name))
        self.hex_label.set_label(color.hex)
        self.rgb_label.set_label(color.rgb_name)
        self.edit_mode = False

        ctrl = Gtk.EventControllerMotion()
        ctrl.connect("enter", lambda _controller, _x, _y: self.button.show() if not self.edit_mode else self.button.hide())
        ctrl.connect("leave", lambda _controller: self.button.hide())
        self.add_controller(ctrl)

        self.button.connect('clicked', lambda _button: self.window.copy_color(self.color.hex))
        self.delete_color_button.connect('clicked', lambda _button:  DeleteColorDialog(self.color, self.palette, self.window, self.db).show())

    def update_edit_view(self):
        self.revealer.set_reveal_child(self.edit_mode)
        self.button.hide()

