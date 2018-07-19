extern crate presentrs;

use presentrs::Server;

fn main() {
    let mut server = Server::new("target/deploy");

    server.run("0.0.0.0:8080").unwrap();
}
