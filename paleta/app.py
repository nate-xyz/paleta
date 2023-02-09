from gi.repository import Gtk, Gio, Adw
from paleta.window import Window
from paleta.database import Database
from paleta.model import Model

class App(Adw.Application):
    """The main application singleton class."""

    def __init__(self):
        super().__init__(application_id='io.github.nate_xyz.Paleta',
                         flags=Gio.ApplicationFlags.FLAGS_NONE)
        self.setup_actions()

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

    def setup_actions(self):
        self.create_action('quit', self.quit, ['<primary>q'])
        self.create_action('about', self.on_about_action)

    def on_about_action(self):
        """Callback for the app.about action."""
        about = Adw.AboutWindow(transient_for=self.props.active_window,
                                application_name='Paleta',
                                application_icon='io.github.nate_xyz.Paleta',
                                developer_name='nate-xyz',
                                version='0.2.1',
                                developers=['nate-xyz'],
                                copyright='© 2023 nate-xyz',
                                license_type=Gtk.License.GPL_3_0_ONLY,
                                website='https://github.com/nate-xyz/paleta',
                                issue_url='https://github.com/nate-xyz/paleta/issues',
        )
        about.add_acknowledgement_section(
            ("Powered by color-thief"),
            [
                "color-thief-py https://github.com/fengsp/color-thief-py",
                "color-thief https://github.com/lokesh/color-thief",
            ]
        )
        about.present()

    def create_action(self, name, callback, shortcuts=None):
        """Add an application action.

        Args:
            name: the name of the action
            callback: the function to be called when the action is
              activated
            shortcuts: an optional list of accelerators
        """
        action = Gio.SimpleAction.new(name, None)
        action.connect("activate", lambda _widget, _parameter: callback())
        self.add_action(action)
        if shortcuts:
            self.set_accels_for_action(f"app.{name}", shortcuts)