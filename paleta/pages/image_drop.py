from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Gst, Pango

@Gtk.Template(resource_path='/io/nxyz/Paleta/image_drop.ui')
class ImageDrop(Adw.Bin):
    __gtype_name__ = 'ImageDrop'

    def __init__(self) -> None:
        super().__init__()
