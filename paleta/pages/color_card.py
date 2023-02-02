from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Gst, Graphene

from paleta.model import Color

@Gtk.Template(resource_path='/io/nxyz/Paleta/color_card.ui')
class ColorCard(Adw.Bin):
    __gtype_name__ = 'ColorCard'

    color_bin = Gtk.Template.Child(name="color_bin")
    hex_label = Gtk.Template.Child(name="hex_label")
    rgb_label = Gtk.Template.Child(name="rgb_label")

    def __init__(self, color: Color) -> None:
        super().__init__()
        self.color = color
        self.color_bin.set_child(ColorSquare(110, color.rgb_name))
        self.hex_label.set_label(color.hex)
        self.rgb_label.set_label(color.rgb_name)



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

        width = self.get_width()
        height = self.get_height()

        rect.init(0, 0, width, height )

        snapshot.append_color(color, rect)