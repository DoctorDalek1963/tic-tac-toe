# list available recipes
default:
	@just -l

# run the benchmarks
bench filter='':
	@cargo bench --features bench {{filter}}
