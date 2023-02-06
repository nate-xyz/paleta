from gi.repository import Adw, GLib, Gio, Gtk, Gdk

from .dropped_image import DroppedImage
from .color_thief_panel import ColorThiefPanel
from os import path

mimes = ['text/uri-list']

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/image_drop_page.ui')
class ImageDropPage(Adw.Bin):
    __gtype_name__ = 'ImageDropPage'

    overlay = Gtk.Template.Child(name="overlay")
    status = Gtk.Template.Child(name="status")
    thief_panel = Gtk.Template.Child(name="thief_panel")

    def __init__(self) -> None:
        super().__init__()
        self.window = None
        self.setup_drop_target()
        self.file_verified = False

    def saturate(self, window, database):
        self.window = window 
        self.thief_panel.saturate(window, database)

    def setup_drop_target(self):
        formats = Gdk.ContentFormats.new(mimes)
        drop_target = Gtk.DropTargetAsync.new(formats=formats, actions=Gdk.DragAction.COPY)
        drop_target.connect('accept', self.on_drag_accept)
        drop_target.connect('drop', self.on_drag_drop)
        
        self.overlay.add_controller(drop_target)

    def on_drag_accept(self, drop_target, drop_value):
        self.file_verified = False
        formats = drop_value.get_formats()
        if contain_mime_types(formats):
            drop_value.read_value_async(Gio.File, GLib.PRIORITY_DEFAULT, None, self.verify_file_valid)
            return True
        return False

    def verify_file_valid(self, drop, task):
        result = drop.read_value_finish(task)
        if not result:
            return
        self.file_verified = path.exists(result.get_path())

    def on_drag_drop(self, drop_target, drop_value, *args):
        if not drop_value:
            self.window.add_error_toast("Unable to read drop.")
            drop_value.finish(0)
            return False
    
        if not self.file_verified:
            self.window.add_error_toast("Unable to verify file on drop, try with the file chooser in the upper left hand corner.", 4)
            drop_value.finish(0)
            return False

        drop_value.read_value_async(Gio.File, GLib.PRIORITY_DEFAULT, None, self.load_value_async)
        return True
        
    def load_value_async(self, drop, task):
        result = drop.read_value_finish(task)
        if not result:
            self.add_error_toast("Unable to read drop.")
            drop.finish(0)
            return
        
        if self.load_image(result.get_path()):
            drop.finish(Gdk.DragAction.COPY)
        else:
            self.add_error_toast("Unable to load image.")
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