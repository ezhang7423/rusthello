run:
	cargo run
dev:
	nodemon -x "make run" -e rs
build:
	cargo build --release
start: build
	./target/release/boy
