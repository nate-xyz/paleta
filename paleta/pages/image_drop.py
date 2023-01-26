from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango

from .drag_overlay import DragOverlay

@Gtk.Template(resource_path='/io/nxyz/Paleta/image_drop.ui')
class ImageDrop(Adw.Bin):
    __gtype_name__ = 'ImageDrop'

    drag_overlay = Gtk.Template.Child(name="drag_overlay")

    def __init__(self) -> None:
        super().__init__()
        self.setup_drop_target()

    def setup_drop_target(self):
        drop_target = Gtk.DropTarget.new(type=Gio.File, actions=Gdk.DragAction.COPY)
        drop_target.connect('drop', self.drag_drop_file)

        self.drag_overlay.set_drop_target(drop_target)

    def drag_drop_file(self, drop_target, file, x, y):
        path = file.get_path()
        print(path)