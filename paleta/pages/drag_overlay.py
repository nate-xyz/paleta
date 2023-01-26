from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango

class DragOverlay(Adw.Bin):
    __gtype_name__ = 'DragOverlay'

    def __init__(self) -> None:
        super().__init__()
        
        self._title = ''
        self._child = None
        self._drop_target = None

        self.overlay = Gtk.Overlay()
        self.revealer = Gtk.Revealer()
        self.status = Adw.StatusPage()
        self.drop_target = Gtk.DropTarget()

        self.construct()

    
    def construct(self):
        self.overlay.set_parent(self)

        self.overlay.add_overlay(self.revealer)

        self.revealer.set_can_target(False)
        self.revealer.set_transition_type(Gtk.RevealerTransitionType.CROSSFADE)
        self.revealer.set_reveal_child(True)

        self.status.set_icon_name("image-missing-symbolic")
        #self.status.add_css_class("drag-overlay-status-page")

        self.revealer.set_child(self.status)

    def set_drop_target(self, drop_target: Gtk.DropTarget):
        if self._drop_target == None:
            return
        
        self.remove_controller(self._drop_target)
                    
        drop_target.connect('notify::current-drop', self.drop_target_notify)

        self.add_controller(drop_target)

    def drop_target_notify(self, drop_target, current_drop):
        self.revealer.set_reveal_child(current_drop)
        

    @GObject.Property(type=str, default=None)
    def title(self):
        return self._title

    @title.setter  
    def title(self, title):
        self.status.set_title(title)
        self._title = title

    @GObject.Property(type=Gtk.Widget, default=None)
    def child(self):
        return self._child

    @child.setter  
    def child(self, child):
        self.overlay.set_child(child)
        self._child = child

    @GObject.Property
    def drop_target(self):
        return self._drop_target

    @drop_target.setter  
    def drop_target(self, drop_target):
        print("drop target setter")
        self.set_drop_target(drop_target)
        self._drop_target = drop_target