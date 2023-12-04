.PHONY: test build release clean

release:
	cargo build --release

build:
	cargo build

test:
	cd types && cargo test && cd ..
	cd api && cargo test && cd ..
	cd runtime && cargo test && cd ..
	cd services && cargo test && cd ..
	cd database && cargo test && cd ..

clean:
	rm -rf target