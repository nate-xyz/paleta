#!/usr/bin/env bash

meson compile -C _builddir --verbose && \
meson devenv -C _builddir ./src/paleta; exit;