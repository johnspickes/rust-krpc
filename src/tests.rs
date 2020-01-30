use crate::rllkrpc::Rpc;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn connects_ok() {

    println!("Connecting...");

    let _rpc = Rpc::connect("10.0.0.126:50000").unwrap();

    println!("Connected!");

    sleep(Duration::new(10,0));

    println!("Disconnecting!");
}
