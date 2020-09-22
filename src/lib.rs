/// Rust low-level kRPC interface
/// This module is a work in progress, planned to implement the low-level interfacing with kRPC
/// through protobufs.  

extern crate protobuf;

mod krpc;

pub mod rllkrpc {

    use std::net::TcpStream;
    use std::net::ToSocketAddrs;

    pub struct Krpc {
        /// TCP stream used for communication with kRPC server
        tcp : TcpStream
    }

    impl Krpc {
        /// Connect to the kRPC server
        pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Krpc> {
            let stream = TcpStream::connect(addr)?;

            // TCP is now connected.
            // Send a connection request message
            let cr = krpc::ConnectionRequest {
                field_type: krpc::ConnectionRequest_Type::RPC,
                client_name: String::from("rkrpc"),
                client_identifier: vec![1],
            };

            let mut os = protobuf::CodedOutputStream::new(&mut stream);
            os.write_message(cr);

            Result::Ok(Krpc {
                tcp : stream
            })
        }
    }
}

mod tests;
