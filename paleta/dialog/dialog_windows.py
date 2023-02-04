from gi.repository import Adw, Gtk

from paleta.model import Palette, Color


import re

@Gtk.Template(resource_path='/io/nxyz/Paleta/save_palette_dialog.ui')
class SavePaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'SavePaletteDialog'

    adw_entry_row = Gtk.Template.Child(name="adw_entry_row")

    name = "Palette"

    def __init__(self, colors, window, database) -> None:
        super().__init__()
        self.colors = colors
        self.window = window
        self.db = database
        self.set_transient_for(self.window)
        self.set_name("Palette #{}".format(self.db.query_n_palettes()+1))

    def set_name(self, name):
        self.name = name 
        self.adw_entry_row.set_text(self.name)

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'save':
            name = self.adw_entry_row.get_text()
            if name == '':
                name = self.name

            if len(self.colors) <= 0:
                return 

            if self.db.add_palette_from_extracted(name, self.colors):
                self.window.add_toast("Created new palette «{}»".format(name))
            else:
                self.window.add_toast("Unable to add new palette «{}»".format(name))


@Gtk.Template(resource_path='/io/nxyz/Paleta/rename_palette_dialog.ui')
class RenamePaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'RenamePaletteDialog'

    adw_entry_row = Gtk.Template.Child(name="adw_entry_row")

    name = "Palette"

    def __init__(self, palette: Palette, window, database) -> None:
        super().__init__()
        self.db = database
        self.window = window
        self.palette = palette

        self.set_transient_for(self.window)
        self.set_name(palette.name)
        self.set_heading("Rename {}?".format(palette.name))
        

    def set_name(self, name):
        self.name = name 
        self.adw_entry_row.set_text(self.name)

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'rename':
            name = self.adw_entry_row.get_text()
            if name == '' or self.palette == None:
                return

            if self.db.rename_palette(self.palette.id, name):
                self.window.add_toast("Renamed palette from «{}» to «{}».".format(self.name, name))
            else:
                self.window.add_toast("Unable to rename palette «{}».".format(self.name))


@Gtk.Template(resource_path='/io/nxyz/Paleta/duplicate_palette_dialog.ui')
class DuplicatePaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'DuplicatePaletteDialog'

    adw_entry_row = Gtk.Template.Child(name="adw_entry_row")

    name = "Palette"

    def __init__(self, palette: Palette, window, database) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.db = database

        self.set_transient_for(self.window)
        self.set_name(palette.name)
        self.set_heading("Duplicate {}?".format(palette.name))

    def set_name(self, name):
        self.name = name 
        self.adw_entry_row.set_text(self.name+' duplicate')

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'duplicate':
            name = self.adw_entry_row.get_text()
            if name == '' or self.palette == None:
                return

            if self.db.duplicate_palette(self.palette.id, name):
                self.window.add_toast("Duplicated palette «{}» to «{}».".format(self.palette.name, name))
            else:
                self.window.add_toast("Unable to duplicate palette «{}».".format(self.palette.name))


@Gtk.Template(resource_path='/io/nxyz/Paleta/delete_palette_dialog.ui')
class DeletePaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'DeletePaletteDialog'

    def __init__(self, palette: Palette, window, database) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.database = database

        self.set_transient_for(self.window)
        self.set_heading("Delete {}?".format(palette.name))

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'delete':
            if self.database.delete_palette(self.palette.id):
                self.window.add_toast("Deleted palette «{}»".format(self.palette.name))
            else:
                self.window.add_toast("Unable to delete palette «{}»".format(self.palette.name))


@Gtk.Template(resource_path='/io/nxyz/Paleta/delete_color_dialog.ui')
class DeleteColorDialog(Adw.MessageDialog):
    __gtype_name__ = 'DeleteColorDialog'

    color_box = Gtk.Template.Child(name="color_box")

    def __init__(self, color: Color, palette: Palette, square, window, database) -> None:
        super().__init__()
        
        self.palette = palette
        self.color = color
        self.window = window
        self.db = database

        self.set_transient_for(self.window)
        self.set_heading("Remove {} from {}?".format(color.hex, palette.name))

        square.set_halign(Gtk.Align.CENTER)
        square.set_hexpand(True)
        self.color_box.append(square)

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'remove':
            if self.palette == None or self.color == None:
                return 

            if self.db.remove_color_from_palette(self.color.id, self.palette.id):
                self.window.add_toast("Remove color {} from palette «{}».".format(self.color.hex, self.palette.name))
            else:
                self.window.add_toast("Unable to remove color {}.".format(self.color.hex))

