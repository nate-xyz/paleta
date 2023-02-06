from gi.repository import Gtk, Gdk, Graphene

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