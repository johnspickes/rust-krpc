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
            let stream = TcpStream::connect(addr)?;

            let mut rpc = Rpc { tcp: stream };

            // TCP is now connected.
            // Send a connection request message

            let mut cr = krpc::ConnectionRequest::new();

            cr.field_type = krpc::ConnectionRequest_Type::RPC;
            cr.client_name = String::from("rkrpc");

            rpc.send(&cr)?;
            let r : krpc::ConnectionResponse = rpc.recv()?;

            if r.get_status() == krpc::ConnectionResponse_Status::OK
            {
                Result::Ok(rpc)
            }
            else
            {
                Result::Err(Error::new(ErrorKind::Other, r.message.clone()))
            }
        }

        /// Send a message
        fn send<M>(&mut self, m: &M) -> protobuf::ProtobufResult<()>
        where M: protobuf::Message {
            let s = m.compute_size();
            // TODO Creating the coded streams with every read or write is probably inefficient
            let mut os = protobuf::CodedOutputStream::new(&mut self.tcp);

            os.write_raw_varint32(s)?;
            m.write_to(&mut os)?;
            os.flush()
        }

        /// Receive a message
        /// TODO How to deal with possibly receiving messages of different types?
        fn recv<M>(&mut self) -> protobuf::ProtobufResult<M>
        where M: protobuf::Message {
            // TODO Creating the coded streams with every read or write is probably inefficient
            let mut is = protobuf::CodedInputStream::new(&mut self.tcp);
            is.read_message()
        }
    }




}

mod tests;
