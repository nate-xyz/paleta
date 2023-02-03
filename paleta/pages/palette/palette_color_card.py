from gi.repository import Gtk, Gdk, Graphene

from paleta.model import Color

@Gtk.Template(resource_path='/io/nxyz/Paleta/palette_color_card.ui')
class PaletteColorCard(Gtk.FlowBoxChild):
    __gtype_name__ = 'PaletteColorCard'

    color_bin = Gtk.Template.Child(name="color_bin")
    hex_label = Gtk.Template.Child(name="hex_label")
    rgb_label = Gtk.Template.Child(name="rgb_label")
    button = Gtk.Template.Child(name="button")

    def __init__(self, color: Color, copy_callback) -> None:
        super().__init__()
        self.color = color
        self.copy_callback = copy_callback
        self.color_bin.set_child(ColorSquare(110, color.rgb_name))
        self.hex_label.set_label(color.hex)
        self.rgb_label.set_label(color.rgb_name)

        ctrl = Gtk.EventControllerMotion()
        ctrl.connect("enter", lambda _controller, _x, _y: self.button.show())
        ctrl.connect("leave", lambda _controller: self.button.hide())
        self.add_controller(ctrl)

        self.button.connect('clicked', lambda _button: self.copy_callback(self.color.hex))

class ColorSquare(Gtk.Widget):
    __gtype_name__ = 'ColorSquare'

    def __init__(self, size, rgb_name) -> None:
        super().__init__()
        self.size = size
        self.rgb_name = rgb_name

    def do_measure(self, orientation, for_size: int):
        return self.size, self.size, -1, -1
            
    def do_snapshot(self, snapshot):
        color = Gdk.RGBA()
        Gdk.RGBA.parse(color, self.rgb_name)

        rect = Graphene.Rect.alloc()
        rect.init(0, 0, self.get_width(), self.get_height())

        snapshot.append_color(color, rect)