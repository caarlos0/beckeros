setup:
	brew install qemu rustup
	rustup-init
	rustup override add nightly
	cargo install bootimage
	rustup component add llvm-tools-preview

build:
	cargo xbuild

build-target:
	cargo xbuild --target x86_64-beckeros.json

img:
	cargo bootimage

run:
	# qemu-system-x86_64 -drive format=raw,file=target/x86_64-beckeros/debug/bootimage-beckeros.bin
	cargo xrun

test:
	cargo xtest
