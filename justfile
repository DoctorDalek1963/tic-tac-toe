# list available recipes
default:
	@just -l

# run the benchmarks
bench filter='':
	cargo bench --features bench {{filter}}

# deploy the web app and docs on the RasPi
deploy: web-deploy doc-deploy

# deploy the web app on the RasPi
web-deploy:
	#!/usr/bin/env bash
	mkdir -p /data/www/tictactoe

	rm -f Trunk.toml
	touch Trunk.toml

	echo '[build]'                      >> Trunk.toml
	echo 'dist = "/data/www/tictactoe"' >> Trunk.toml
	echo 'public_url = "/tictactoe/"'   >> Trunk.toml
	echo 'release = true'               >> Trunk.toml

	trunk build
	rm -f Trunk.toml

# build the docs and optionally open them
doc-build open='':
	cargo doc --no-deps --document-private-items --workspace --release --target-dir target {{open}}

# build and open the docs
doc-open: (doc-build "--open")

# deploy the docs on the RasPi
doc-deploy: doc-build
	mkdir -p /data/www/tictactoe
	mv target/doc/ /data/www/tictactoe/doc
