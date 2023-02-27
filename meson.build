project('paleta', 'rust', 
          version: '0.3.1',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=3', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')

dependency('gtk4', version: '>= 4.6.0')
dependency('libadwaita-1', version: '>= 1.2.0')

cargo = find_program('cargo', required: true)
cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

subdir('data')
subdir('src')
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