from gi.repository import Adw, GLib, Gtk, GdkPixbuf
from PIL import Image

class DroppedImage(Adw.Bin):
    __gtype_name__ = 'DroppedImage'

    def __init__(self, image_path) -> None:
        super().__init__()
        self.set_halign(Gtk.Align.CENTER)
        self.set_valign(Gtk.Align.CENTER)
        self.load_image(image_path)

    def load_image(self, image_path):
        self.image_path = image_path
        source_image = Image.open(image_path)
        width, height = source_image.size
        bytes_ = source_image.tobytes("raw", "RGBA")
        glib_bytes = GLib.Bytes.new(bytes_)
        pixbuf = GdkPixbuf.Pixbuf.new_from_bytes(glib_bytes, GdkPixbuf.Colorspace.RGB, True, 8, width, height, width*4)

        self.picture = Gtk.Picture.new_for_pixbuf(pixbuf)
        self.set_child(self.picture)

    