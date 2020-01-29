/// Rust low-level kRPC interface
/// This module is a work in progress, planned to implement the low-level interfacing with kRPC
/// through protobufs.  

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
            Result::Ok(Krpc {
                tcp : stream
            })
        }
    }
}

mod tests;
