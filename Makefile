.PHONY: test build build-frontend release clean frontend frontend-install frontend-dev

release: frontend
	cargo build --release

build:
	cargo build

build-frontend: frontend
	cargo build

test:
	cargo test

clean:
	rm -rf target
	rm -rf frontend/dist
	rm -rf frontend/node_modules

frontend-install:
	cd frontend && npm install

frontend:
	cd frontend && npm run build

frontend-dev:
	cd frontend && npm run dev