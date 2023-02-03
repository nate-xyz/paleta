pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
moduledir = join_paths(pkgdatadir, 'paleta')

gnome.compile_resources('paleta',
  'paleta.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

python = import('python')

conf = configuration_data()
conf.set('PYTHON', python.find_installation('python3').path())
conf.set('VERSION', meson.project_version())
conf.set('localedir', join_paths(get_option('prefix'), get_option('localedir')))
conf.set('pkgdatadir', pkgdatadir)
conf.set('application_id', application_id)

configure_file(
  input: 'paleta.in',
  output: 'paleta',
  configuration: conf,
  install: true,
  install_dir: get_option('bindir')
)


paleta_sources = [
    '__init__.py',
    'main.py',
    'app.py',
    'window.py',
    'model.py',
    'database.py',
]

subdir('pages')

install_data(paleta_sources, install_dir: moduledir)