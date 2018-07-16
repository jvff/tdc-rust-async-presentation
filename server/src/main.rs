extern crate presentrs;

use presentrs::Server;

fn main() {
    let mut server = Server;

    server.run("0.0.0.0:8080").unwrap();
}
