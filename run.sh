#!/usr/bin/env bash

meson compile -C _builddir && \
meson devenv -C _builddir ./paleta/paleta; exit; 