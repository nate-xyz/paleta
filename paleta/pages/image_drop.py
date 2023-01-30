from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango

from .drag_overlay import DragOverlay
from .image import PaletaImage

mimes = ['text/uri-list']

@Gtk.Template(resource_path='/io/nxyz/Paleta/image_drop.ui')
class ImageDropPage(Adw.Bin):
    __gtype_name__ = 'ImageDropPage'

    overlay = Gtk.Template.Child(name="overlay")
    revealer = Gtk.Template.Child(name="revealer")
    status = Gtk.Template.Child(name="status")

    def __init__(self) -> None:
        super().__init__()
        self.setup_drop_target()

    def setup_drop_target(self):
        formats = Gdk.ContentFormats.new(mimes)
        drop_target = Gtk.DropTargetAsync.new(formats=formats, actions=Gdk.DragAction.COPY)
        drop_target.connect('accept', self.on_drag_accept)
        drop_target.connect('drop', self.on_drag_drop)
        
        self.overlay.add_controller(drop_target)


    def on_drag_accept(self, drop_target, drop_value):
        print('on_drag_accept', drop_value)
        
        #return True
        formats = drop_value.get_formats()
        if contain_mime_types(formats):
            drop_value.read_value_async(Gio.File, GLib.PRIORITY_DEFAULT, None, self.verify_file_valid)
            return True
        return False


    def verify_file_valid(self, drop, task):
        result = drop.read_value_finish(task)
        if not result:
            print("reading value failed")
            return
        path = result.get_path()
        print(path)

    def on_drag_drop(self, drop_target, drop_value, *args):
        print('on_drag_drop')

        if not drop_value:
            print("Drop value error")
            drop_value.finish(0)
            return False

        drop_value.read_value_async(Gio.File, GLib.PRIORITY_DEFAULT, None, self.load_value_async)
        return True
        

    def load_value_async(self, drop, task):
        result = drop.read_value_finish(task)
        if not result:
            print("reading value failed")
            drop.finish(0)
            return
        
        if self.load_image(result.get_path()):
            drop.finish(Gdk.DragAction.COPY)
        else:
            drop.finish(0)

    
    def load_image(self, uri):
        try:
            image = PaletaImage()
            image.load_image(uri)
            self.overlay.set_child(image)
            self.revealer.set_reveal_child(False)
            return True
        except:
            return False


def contain_mime_types(formats):
    if formats is not None:
        return True in (formats.contain_mime_type(m) for m in mimes)
    return False