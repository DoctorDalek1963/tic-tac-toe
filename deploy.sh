#!/usr/bin/env sh

mkdir -p /data/www/tictactoe

# Web app
git restore Trunk.toml

echo 'dist = "/data/www/tictactoe"' >> Trunk.toml
echo 'public_url = "/tictactoe/"'   >> Trunk.toml
echo 'release = true'               >> Trunk.toml

trunk build
git restore Trunk.toml
