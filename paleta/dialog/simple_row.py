from gi.repository import Adw, Gio, Gtk

from paleta.model import Color
from paleta.pages import ColorSquare

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/simple_palette_row.ui')
class SimplePaletteRow(Gtk.ListBoxRow):
    __gtype_name__ = 'SimplePaletteRow'

    flow_box = Gtk.Template.Child(name="flow_box")

    def __init__(self, model, set_color) -> None:
        super().__init__()
        self.colors = model.get_colors().items() 
        self.set_color = set_color
        self.list_store = Gio.ListStore(item_type=Color)
        self.flow_box.bind_model(self.list_store, self.flowbox_factory)
        self.flow_box.set_min_children_per_line(len(self.colors))
        self.flow_box.set_max_children_per_line(len(self.colors))
        self.update_view()

    def update_view(self):
        self.list_store.remove_all()
        for _, color in self.colors:
            self.list_store.append(color)
        
    def flowbox_factory(self, color):
        return SimpleColorCard(color, self.set_color)

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/simple_color_card.ui')
class SimpleColorCard(Gtk.FlowBoxChild):
    __gtype_name__ = 'SimpleColorCard'

    color_bin = Gtk.Template.Child(name="color_bin")
    hex_label = Gtk.Template.Child(name="hex_label")
    rgb_label = Gtk.Template.Child(name="rgb_label")
    button = Gtk.Template.Child(name="button")

    def __init__(self, color: Color, set_color) -> None:
        super().__init__()
        self.color = color
        self.set_color = set_color
        self.color_bin.set_child(ColorSquare(110, color.rgb_name))
        self.hex_label.set_label(color.hex)
        self.rgb_label.set_label(color.rgb_name)
        self.edit_mode = False

        ctrl = Gtk.EventControllerMotion()
        ctrl.connect("enter", lambda _controller, _x, _y: self.button.show() if not self.edit_mode else self.button.hide())
        ctrl.connect("leave", lambda _controller: self.button.hide())
        self.add_controller(ctrl)

        self.button.connect('clicked', lambda _button: self.set_color(self.color))
