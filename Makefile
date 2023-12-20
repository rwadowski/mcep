.PHONY: test build release clean

release:
	cargo build --release

build:
	cargo build

test:
	cargo test

clean:
	rm -rf target