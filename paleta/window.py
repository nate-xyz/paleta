# window.py
#
# Copyright 2023 nate-xyz
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# SPDX-License-Identifier: GPL-3.0-or-later

from gi.repository import Adw, Gtk, Gdk, Gio

# from paleta.pages.image_drop import ImageDropPage
# from paleta.pages.palettes import PalettePage
from .pages import ImageDropPage, PalettePage
from .util import SUCCESS_GREEN, ERROR_RED

import os
import html

image_mime_types = ['image/jpeg', 'image/png', 'image/tiff', 'image/webp']

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/window.ui')
class Window(Adw.ApplicationWindow):
    __gtype_name__ = 'Window'

    toast_overlay = Gtk.Template.Child(name="toast_overlay")
    header_bar = Gtk.Template.Child(name="header_bar")
    view_switcher_title = Gtk.Template.Child(name="view-switcher-title")
    stack = Gtk.Template.Child(name="stack")
    open_image_button = Gtk.Template.Child(name="open_image_button")
    image_drop_page = Gtk.Template.Child(name="image_drop_page")
    palette_page = Gtk.Template.Child(name="palette_page")
    edit_palette_button = Gtk.Template.Child(name="edit_palette_button")

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        app = kwargs['application']
        self.add_dialog()
        help_overlay = Gtk.Builder\
            .new_from_resource('/io/github/nate_xyz/Paleta/help-overlay.ui')\
            .get_object('help_overlay')
        self.set_help_overlay(help_overlay)
        
        self.setup_actions(app)

        self.db = app.db 
        self.db.window = self
        
        self.model = app.model
        
        self.saturate()
        
        if not self.db.try_loading_database():
            self.add_error_toast(_("Unable to load database."))
    
        self.clipboard = Gdk.Display.get_default().get_clipboard()
        self.setup_switcher_button()

        self.stack.connect('notify::visible-child-name', self.on_stack_switch)
        self.edit_palette_button.connect('clicked', lambda _button: self.palette_page.toggle_edit_mode())

    def setup_actions(self, app):
        app.set_accels_for_action("win.show-help-overlay", ['<Primary>question'])
        self.create_action(app, 'image-open', self.open_image_dialog.show, ['<Primary>o'])
        self.create_action(app, 'image-palette-extract', self.image_drop_page.thief_panel.start_extraction, ['<Primary>e', '<Primary>Return'])
        self.create_action(app, 'image-palette-save', self.image_drop_page.thief_panel.save_palette, ['<Primary>s'])
        self.create_action(app, 'image-page', self.go_to_image_drop_page, ['<Primary>i', '<Primary>d'])
        self.create_action(app, 'palette-edit-mode', self.palette_page.toggle_edit_mode, ['<Primary>t', '<Primary>m'])
        self.create_action(app, 'palette-add', self.palette_page.show_new_palette_dialog, ['<Primary>n','<Primary>a'])
        self.create_action(app, 'palette-page', self.go_to_palette_page, ['<Primary>p'])

    def on_stack_switch(self, stack, child_name):
        if stack.get_visible_child_name() == 'palette-stack-page' and len(self.model.get_palettes()) != 0:
            self.edit_palette_button.show()
        else:
            self.edit_palette_button.hide()
    
    def saturate(self):
        self.image_drop_page.saturate(self, self.db)
        self.palette_page.saturate(self, self.db, self.model)

    def setup_switcher_button(self):
        #adw switcher buttons
        squeezer = self.view_switcher_title.observe_children()[0]
        view_switcher = squeezer.observe_children()[0]
        self.switcher_buttons =  view_switcher.observe_children()
        #self.drop_button, self.palette_button = self.switcher_buttons
        for button in self.switcher_buttons:
            button.connect("clicked", self.switcher_button)

    def switcher_button(self, button):
        if self.check_switcher_title_bug(button):
            self.replace_switcher()

    def check_switcher_title_bug(self, active_button):
        error_string = 'button.flat.horizontal.toggle:active:dir(ltr)'
        for button in self.switcher_buttons:
            style_context = button.get_style_context()
            check_string = style_context.to_string(Gtk.StyleContextPrintFlags.SHOW_CHANGE).split(' ')[0]
            if button != active_button and error_string==check_string:
                return True
        return False
    
    def replace_switcher(self):        
        self.header_bar.remove(self.view_switcher_title)
        self.view_switcher_title = Adw.ViewSwitcherTitle()
        self.view_switcher_title.set_stack(self.stack)
        self.header_bar.set_title_widget(self.view_switcher_title)
        
        self.switcher_buttons =  self.view_switcher_title.observe_children()[0].observe_children()[0].observe_children()

        for button in self.switcher_buttons:
            button.connect("clicked", self.switcher_button)

    def add_dialog(self):
        self.open_image_dialog = Gtk.FileChooserNative.new(title=_("Select an Image File"), 
                                                        parent=self, 
                                                        action=Gtk.FileChooserAction.OPEN, 
                                                        accept_label=_("Open Image"))

        f = Gtk.FileFilter()
        f.set_name(_("Image files"))
        for m in image_mime_types:
            f.add_mime_type(m)

        self.open_image_dialog.connect("response", self.open_response)
        self.open_image_dialog.add_filter(f)
        self.open_image_button.connect("clicked", lambda _button: self.open_image_dialog.show())
        self.image_drop_page.open_image_button.connect("clicked", lambda _button: self.open_image_dialog.show())

    def open_response(self, dialog, response):
        if response == Gtk.ResponseType.ACCEPT:
            image_uri = dialog.get_file().get_path()
            if self.image_drop_page.load_image(image_uri):
                self.go_to_image_drop_page()
                self.open_image_toast(image_uri)
            else:
                self.error_image_toast(image_uri)

    def add_toast(self, title: str, timeout: int = 1):
        toast = Adw.Toast.new(html.escape(title))
        toast.set_timeout(timeout)
        self.toast_overlay.add_toast(toast)

    def add_toast_markup(self, title: str, timeout: int = 1):
        toast = Adw.Toast.new(title)
        toast.set_timeout(timeout)
        self.toast_overlay.add_toast(toast)

    def error_image_toast(self, uri):
        base_name = os.path.basename(uri)
        # Translators: Do not replace {}
        self.add_error_toast("Could not open image: {}".format(base_name), 3)

    def open_image_toast(self, uri):
        base_name = html.escape(os.path.basename(uri))
        # Translators: Do not replace {base_name}, {SUCCESS_GREEN}, or the span tags, only translate "Opened image:"
        self.add_toast_markup(f"<span foreground={SUCCESS_GREEN}>Opened image:</span>  {base_name}")

    def add_success_toast(self, verb: str, msg: str, timeout: int = 1):
        toast = Adw.Toast.new(f"<span foreground={SUCCESS_GREEN}>{verb}</span> {html.escape(msg)}")
        toast.set_timeout(timeout)
        self.toast_overlay.add_toast(toast)

    def add_error_toast(self, error: str, timeout: int = 1):
        # Translators: Only replace "Error!"
        toast = Adw.Toast.new(_(f"<span foreground={ERROR_RED}>Error!</span> {html.escape(error)}"))
        toast.set_timeout(timeout)
        self.toast_overlay.add_toast(toast)

    def add_color_toast(self, hex_name, palette_name):
        # Translators: Do not replace {hex_name}, only translate "Added color" and "to palette"
        self.add_toast_markup(_(f"Added color <span foreground=\"{hex_name}\">{hex_name}</span> to palette «{html.escape(palette_name)}»."))

    def remove_color_toast(self, hex_name, palette_name):
        # Translators: Do not replace {hex_name}, only translate "Removed color" and "from palette"
        self.add_toast_markup(_(f"Removed color <span foreground=\"{hex_name}\">{hex_name}</span> from palette «{html.escape(palette_name)}»."))

    def copy_color(self, hex_name):
        # Translators: Do not replace {hex_name}, only translate "Copied color" and "to clipboard!"
        self.add_toast_markup(_(f"Copied color <span foreground=\"{hex_name}\">{hex_name}</span> to clipboard!"))
        self.clipboard.set(hex_name)

    def go_to_image_drop_page(self):
        if self.stack.get_visible_child_name() != 'drop-stack-page':
            self.stack.set_visible_child(self.image_drop_page)

    def go_to_palette_page(self):
        if self.stack.get_visible_child_name() != 'palette-stack-page':
            self.stack.set_visible_child(self.palette_page)

    def create_action(self, app, name, callback=None, shortcuts=None):
        """Add a window action.

        Args:
            name: the name of the action
            callback: the function to be called when the action is
              activated
            shortcuts: an optional list of accelerators
        """
        action = Gio.SimpleAction.new(name, None)
        if callback != None:
            action.connect("activate", lambda _widget, _parameter: callback())
        self.add_action(action)
        if shortcuts:
            app.set_accels_for_action(f"win.{name}", shortcuts)