use crate::rllkrpc::Rpc;

#[test]
fn connects_ok() {
    let _rpc = Rpc::connect("10.0.0.126:50000").unwrap();
}
