from gi.repository import Adw, GLib, Gio, Gtk, Gdk

from .dropped_image import DroppedImage
from .color_thief_panel import ColorThiefPanel

mimes = ['text/uri-list']

@Gtk.Template(resource_path='/io/nxyz/Paleta/image_drop_page.ui')
class ImageDropPage(Adw.Bin):
    __gtype_name__ = 'ImageDropPage'

    overlay = Gtk.Template.Child(name="overlay")
    status = Gtk.Template.Child(name="status")
    thief_panel = Gtk.Template.Child(name="thief_panel")

    def __init__(self) -> None:
        super().__init__()
        self.window = None
        self.setup_drop_target()

    def set_win(self, window):
        self.thief_panel.set_win(window)
        self.window = window

    def set_db(self, db):
        self.thief_panel.set_db(db)

    def setup_drop_target(self):
        formats = Gdk.ContentFormats.new(mimes)
        drop_target = Gtk.DropTargetAsync.new(formats=formats, actions=Gdk.DragAction.COPY)
        drop_target.connect('accept', self.on_drag_accept)
        drop_target.connect('drop', self.on_drag_drop)
        
        self.overlay.add_controller(drop_target)


    def on_drag_accept(self, drop_target, drop_value):
        print('on_drag_accept', drop_value)
    
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
            self.thief_panel.set_image(DroppedImage(uri))
            self.status.hide()
            self.window.open_image_toast(uri)
            return True
        except Exception as e:
            print(e)
            self.window.error_image_toast(uri)
            return False


def contain_mime_types(formats):
    if formats is not None:
        return True in (formats.contain_mime_type(m) for m in mimes)
    return False