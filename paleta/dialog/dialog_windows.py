from gi.repository import Adw, Gtk

from paleta.model import Palette

@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/save_palette_dialog.ui')
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
                # Translators: Do not replace {name}
                self.window.add_success_toast(_("Saved!"), _(f"New palette: «{name}»"))
                self.window.go_to_palette_page()
            else:
                # Translators: Do not replace {name}
                self.window.add_error_toast(_(f"Unable to add new palette «{name}»"))


@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/rename_palette_dialog.ui')
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
        
        # Translators: Do not replace {palette.name}
        self.set_heading(_(f"Rename {palette.name}?"))
        

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
                # Translators: Do not replace {self.name} or {name}
                self.window.add_success_toast(_("Renamed!"), _(f"Changed name from «{self.name}» to «{name}»."))
            else:
                # Translators: Do not replace {self.name}
                self.window.add_error_toast(_(f"Unable to rename palette «{self.name}»."))


@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/duplicate_palette_dialog.ui')
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

        # Translators: Do not replace {palette.name}
        self.set_heading(_(f"Duplicate {palette.name}?"))

    def set_name(self, name):
        self.name = name 

        self.adw_entry_row.set_text(_(f"{self.name} duplicate"))

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'duplicate':
            name = self.adw_entry_row.get_text()
            if name == '' or self.palette == None:
                return

            if self.db.duplicate_palette(self.palette.id, name):
                # Translators: Do not replace {self.palette.name} or {name}
                self.window.add_success_toast(_("Duplicated!"), _(f"Copied «{self.palette.name}» to «{name}»."))
            else:
                # Translators: Do not replace {self.palette.name}
                self.window.add_error_toast(_(f"Unable to duplicate palette «{self.palette.name}»."))


@Gtk.Template(resource_path='/io/github/nate_xyz/Paleta/delete_palette_dialog.ui')
class DeletePaletteDialog(Adw.MessageDialog):
    __gtype_name__ = 'DeletePaletteDialog'

    def __init__(self, palette: Palette, window, database) -> None:
        super().__init__()
        self.palette = palette
        self.window = window
        self.database = database

        self.set_transient_for(self.window)

        # Translators: Do not replace {palette.name}
        self.set_heading(_(f"Delete {palette.name}?"))

    @Gtk.Template.Callback()
    def dialog_response(self, dialog, response):
        if response == 'delete':
            if self.database.delete_palette(self.palette.id):
                # Translators: Do not replace {self.palette.name}
                self.window.add_success_toast(_("Removed"), _(f"palette: «{self.palette.name}»."))
            else:
                # Translators: Do not replace {self.palette.name}
                self.window.add_error_toast(_(f"Unable to delete palette «{self.palette.name}»."))


