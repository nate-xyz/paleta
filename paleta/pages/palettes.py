from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Gst, Pango

@Gtk.Template(resource_path='/io/nxyz/Paleta/palettes.ui')
class Palettes(Adw.Bin):
    __gtype_name__ = 'Palettes'

    def __init__(self) -> None:
        super().__init__()
