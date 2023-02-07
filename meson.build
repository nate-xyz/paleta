project('paleta',
          version: '0.1.1',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=3', 'werror=false', ],
)

dependency('gtk4', version: '>= 4.6.0')
dependency('libadwaita-1')

i18n = import('i18n')
gnome = import('gnome')

application_id = 'io.github.nate_xyz.Paleta'

subdir('data')
subdir('paleta')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)

summary({
    'prefix': get_option('prefix'),
    'libdir': get_option('libdir'),
    'datadir': get_option('datadir'),
    'bindir': get_option('bindir'),
  },
  section: 'Directories',
)