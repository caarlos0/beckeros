build:
	# the "none" target triple is to say we don't want an underlying OS.
	# this prevents rust trying to link the C runtime
	cargo build --target thumbv7em-none-eabihf
