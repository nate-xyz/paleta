project('paleta', 'rust',
          version: '0.3.1',
    meson_version: '>= 0.62.0',
  default_options: ['warning_level=3',
                    'werror=false',
                   ],
)

i18n = import('i18n')
gnome = import('gnome')

cargo_bin = find_program('cargo', required: true)
git_bin = find_program('git', required: false)

cargo_opt = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'debug'
  if git_bin.found()
    VCS_TAG = run_command('git', 'rev-parse', '--short', 'HEAD', check: true).stdout().strip()
  endif
  if VCS_TAG == ''
    VERSION_SUFFIX = '-devel'
  else
    VERSION_SUFFIX = '-@0@'.format(VCS_TAG)
  endif
  rust_target = 'debug'
  #APPLICATION_ID = '@0@.Devel'.format(PROJECT_RDNN_NAME)
else
  cargo_opt += [ '--release' ]
  rust_target = 'release'
  VERSION_SUFFIX = ''
  #APPLICATION_ID = PROJECT_RDNN_NAME
endif

dependency('glib-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.5.0')
dependency('libadwaita-1', version: '>= 1.2.0')

pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())

conf = configuration_data()
conf.set('VERSION', meson.project_version() + VERSION_SUFFIX)
conf.set('GETTEXT_PACKAGE', 'paleta')
conf.set('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set('PKGDATADIR', pkgdatadir)

subdir('data')
subdir('src')
subdir('po')

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock'
)

sources = [cargo_sources, rust_sources]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env', cargo_env,
    cargo_bin, 'build', cargo_opt,
    '&&', 'cp', 'src' / rust_target / meson.project_name(), 'src' / '@OUTPUT@',
  ]
)

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
