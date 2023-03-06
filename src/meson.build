gnome = import('gnome')

gnome.compile_resources('paleta',
  'paleta.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir
)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)

rust_sources = files(
  'application.rs',
  'config.rs',
  'database.rs',
  'main.rs',
  'util.rs',
  'window.rs'
)

subdir('model')
subdir('dialog')
subdir('pages')
