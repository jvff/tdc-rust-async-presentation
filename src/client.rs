extern crate presentrs;
extern crate yew;

use presentrs::Presentrs;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Presentrs>::new().mount_to_body();
    yew::run_loop();
}
