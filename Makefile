.PHONY: run all client server

all: client server

run: all
	target/debug/server
	
client:
	cargo-web deploy -p client
	cp -a static/* target/deploy/
	cp -a slides target/deploy/

server:
	cargo build -p server
