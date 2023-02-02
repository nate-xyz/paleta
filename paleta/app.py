from gi.repository import Gtk, Gio, Adw
from paleta.window import Window
from paleta.database import Database
from paleta.model import Model

class App(Adw.Application):
    """The main application singleton class."""

    def __init__(self):
        super().__init__(application_id='io.nxyz.Paleta',
                         flags=Gio.ApplicationFlags.FLAGS_NONE)
        self.create_action('quit', self.quit, ['<primary>q'])
        self.create_action('about', self.on_about_action)
        self.create_action('preferences', self.on_preferences_action)

        self.db = Database()
        self.model = Model(self.db)
        self.db.model = self.model

    def do_activate(self):
        """Called when the application is activated.

        We raise the application's main window, creating it if
        necessary.
        """
        try:
            win = self.props.active_window
            if not win:
                win = Window(application=self)
            win.present()
        except Exception as e:
            print(e)

    def on_about_action(self, widget, _):
        """Callback for the app.about action."""
        about = Adw.AboutWindow(transient_for=self.props.active_window,
                                application_name='paleta',
                                application_icon='io.nxyz.Paleta',
                                developer_name='nate-xyz',
                                version='0.1.0',
                                developers=['nate-xyz'],
                                copyright='© 2023 nate-xyz')
        about.present()

    def on_preferences_action(self, widget, _):
        """Callback for the app.preferences action."""
        print('app.preferences action activated')

    def create_action(self, name, callback, shortcuts=None):
        """Add an application action.

        Args:
            name: the name of the action
            callback: the function to be called when the action is
              activated
            shortcuts: an optional list of accelerators
        """
        action = Gio.SimpleAction.new(name, None)
        action.connect("activate", callback)
        self.add_action(action)
        if shortcuts:
            self.set_accels_for_action(f"app.{name}", shortcuts)