/// Rust low-level kRPC interface
/// This module is a work in progress, planned to implement the low-level interfacing with kRPC
/// through protobufs.  

extern crate protobuf;

mod krpc;

pub mod rllkrpc {

    use std::net::TcpStream;
    use std::net::ToSocketAddrs;
    use std::io::{Error, ErrorKind};

    use super::krpc;

    pub struct Rpc {
        /// TCP stream used for communication with kRPC server
        tcp : TcpStream
    }

    impl Rpc {
        /// Connect to the kRPC server
        pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Rpc> {
            let mut stream = TcpStream::connect(addr)?;

            // TCP is now connected.
            // Send a connection request message
            let mut cr = krpc::ConnectionRequest::new();

            cr.field_type = krpc::ConnectionRequest_Type::RPC;
            cr.client_name = String::from("rkrpc");

            let r : krpc::ConnectionResponse;
            {
                send(&cr, &mut stream)?;

                let mut is = protobuf::CodedInputStream::new(&mut stream);
                r = is.read_message()?;
            }

            if r.get_status() == krpc::ConnectionResponse_Status::OK
            {
                Result::Ok(Rpc {
                    tcp : stream
                })
            }
            else
            {
                Result::Err(Error::new(ErrorKind::Other, r.message.clone()))
            }
        }
    }

    fn send<M>(m: &M, stream: &mut TcpStream) -> protobuf::ProtobufResult<()>
    where M: protobuf::Message {
        let s = m.compute_size();
        let mut os = protobuf::CodedOutputStream::new(stream);

        os.write_raw_varint32(s)?;
        m.write_to(&mut os)?;
        os.flush()
    }



}

mod tests;
