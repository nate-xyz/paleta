#!/usr/bin/env bash

pip install -r requirements.txt && \
meson setup _builddir && \
sh run.sh 