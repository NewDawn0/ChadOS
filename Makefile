CC=cargo build

.PHONY: clean setup reset
setup:
	rustup default nightly
	rustup target add x86_64-unknown-none

build:
	$(CC) --release 

reset:
	rustup default stable
