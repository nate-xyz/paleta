#!/usr/bin/env bash

meson compile -C _builddir --verbose && \
meson devenv -C _builddir ./paleta/paleta; exit; 