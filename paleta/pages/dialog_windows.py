from gi.repository import Adw, Gtk

@Gtk.Template(resource_path='/io/nxyz/Paleta/save_dialog.ui')
class SaveDialog(Adw.MessageDialog):
    __gtype_name__ = 'SaveDialog'

    title_adw_entry = Gtk.Template.Child(name="title_adw_entry")

    name = "Palette"

    def __init__(self, database, window, colors) -> None:
        super().__init__()
        self.db = database
        self.window = window
        self.colors = colors

        self.set_transient_for(self.window)
        self.reset_name("Palette #{}".format(self.db.query_n_palettes()+1))

    def reset_name(self, name):
        self.name = name 
        self.title_adw_entry.set_text(self.name)

    @Gtk.Template.Callback()
    def save_dialog_response(self, dialog, response):
        if response == 'save':
            name = self.title_adw_entry.get_text()
            if name == '':
                name = self.name

            if len(self.colors) <= 0:
                return 

            self.db.add_palette(name, self.colors)
            self.window.add_toast("Created new palette \"{}\"".format(name))