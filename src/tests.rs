#[allow(unused_imports)]
use crate::rllkrpc::{Tcp, Streams};
#[allow(unused_imports)]
use std::thread::sleep;
#[allow(unused_imports)]
use std::time::Duration;

#[test]
fn connects_ok() {

    println!("Connecting...");

    let mut _rpc = Tcp::connect("10.0.0.126:50000").unwrap();
    let _streams = Streams::connect(&mut _rpc).unwrap();

    println!("Connected!");

    sleep(Duration::new(10,0));

    println!("Disconnecting!");
}
