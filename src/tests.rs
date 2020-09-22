use crate::rllkrpc::Krpc;

#[test]
fn connects_ok() {
    let _rpc = Krpc::connect("127.0.0.1:50000").unwrap();
}
