#!/usr/bin/bash

read -p "Do you want to do a clean compilation? [n/y] " answer

if [[ "$answer" == "y" ]]; then
    rm -r _builddir
fi

meson setup _builddir
meson configure _builddir -Dbuildtype=debug

meson compile -C _builddir --verbose
meson devenv -C _builddir ./src/paleta
