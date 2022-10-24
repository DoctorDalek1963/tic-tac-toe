#!/usr/bin/env sh

mkdir -p /data/www/tictactoe

rm -f Trunk.toml

touch Trunk.toml
echo '[build]'                      >> Trunk.toml
echo 'dist = "/data/www/tictactoe"' >> Trunk.toml
echo 'public_url = "/tictactoe/"'   >> Trunk.toml
echo 'release = true'               >> Trunk.toml

trunk build
rm -f Trunk.toml
