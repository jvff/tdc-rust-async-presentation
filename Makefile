.PHONY: run all client server

all: client server

run: all
	target/debug/server
	
client:
	cd client; trunk build

server:
	cargo build -p server --release
