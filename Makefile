
.PHONY: test

test:
	cd types && cargo test && cd ..
	cd api && cargo test && cd ..
	cd runtime && cargo test && cd ..
	cd services && cargo test && cd ..
	cd database && cargo test && cd ..