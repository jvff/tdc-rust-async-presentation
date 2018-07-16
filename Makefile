.PHONY: run all client server

all: client server

run: all
	target/debug/server
	
client:
	rm -f static/*.js static/*.wasm
	cargo web deploy -p client
	cp target/deploy/*.js target/deploy/*.wasm static

server:
	cargo build -p server
