#!/usr/bin/env sh

mkdir -p /data/www/tictactoe

# Web app
rm -f Trunk.toml
touch Trunk.toml

echo '[build]'                      >> Trunk.toml
echo 'dist = "/data/www/tictactoe"' >> Trunk.toml
echo 'public_url = "/tictactoe/"'   >> Trunk.toml
echo 'release = true'               >> Trunk.toml

trunk build
rm -f Trunk.toml

# Docs
cargo doc --no-deps --document-private-items --workspace --release --target-dir target
mv target/doc/ /data/www/tictactoe/doc
