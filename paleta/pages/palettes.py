from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango

@Gtk.Template(resource_path='/io/nxyz/Paleta/palettes.ui')
class PalettePage(Adw.Bin):
    __gtype_name__ = 'PalettePage'

    def __init__(self) -> None:
        super().__init__()
