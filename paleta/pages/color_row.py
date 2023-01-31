from gi.repository import Adw, GLib, Gio, Gtk, Gdk, GObject, GdkPixbuf, Pango
from PIL import Image, ImageFilter, ImageChops, ImageDraw
import re

@Gtk.Template(resource_path='/io/nxyz/Paleta/color_row.ui')
class ColorRow(Gtk.ListBoxRow):
    __gtype_name__ = 'ColorRow'

    row_box = Gtk.Template.Child(name="row_box")
    hex_name_label = Gtk.Template.Child(name="hex_name_label")
    rgb_name_label = Gtk.Template.Child(name="rgb_name_label")
    copy_icon = Gtk.Template.Child(name="copy_icon")


    def __init__(self) -> None:
        super().__init__()
        ctrl = Gtk.EventControllerMotion()
        ctrl.connect("enter", self.on_enter)
        ctrl.connect("leave", self.on_leave)
        self.add_controller(ctrl)


    def load_color(self, color):
        rgba = Gdk.RGBA()
        success = rgba.parse(color.rgb_name)
        if success:
            color_button = Gtk.ColorButton.new_with_rgba(rgba)
            color_button.connect('color-set', self.set_from_button)
            color_button.props.show_editor = True
            self.row_box.prepend(color_button)
        
        
        self.hex_name_label.set_label(color.hex_name)
        self.rgb_name_label.set_label(color.rgb_name)

        self.hex_name = color.hex_name

    def set_from_button(self, button):
        new_color = button.get_rgba()
        rgb_name = new_color.to_string()
        self.rgb_name_label.set_label(rgb_name)
        rgb = [int(i) for i in re.search('\(([^)]+)', rgb_name).group(1).split(',')]
        self.hex_name = rgb_to_hex(*rgb)
        self.hex_name_label.set_label("#{}".format(self.hex_name))



    def on_enter(self, _controller, x, y):
        self.copy_icon.show()

    def on_leave(self, _controller):
        self.copy_icon.hide()



class PaletaColor(GObject.GObject):
    __gtype_name__ = "PaletaColor"

    def __init__(self) -> None:
        super().__init__()
    
    def add_rgb(self, rgb_tuble):
        self.rgb_name = "rgb{}".format(rgb_tuble)
        self.hex_name = "#{}".format(rgb_to_hex(*rgb_tuble))


def rgb_to_hex(r, g, b):
  return ('{:X}{:X}{:X}').format(r, g, b)