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

    pub struct Tcp {
        /// TCP stream used for communication with kRPC server
        in_stream : TcpStream,
        out_stream : TcpStream
    }

    impl Tcp {
        /// Connect to the kRPC server
        pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<Tcp> {
            let in_stream = TcpStream::connect(addr)?;
            let out_stream = in_stream.try_clone()?;

            let rpc = Tcp {
                in_stream : in_stream,
                out_stream : out_stream
            };

            Result::Ok(rpc)
        }

        /// Get streams
        pub fn get_streams(&mut self) -> std::io::Result<Streams> {
            Streams::new(self)
        }
    }

    pub struct Streams<'a> {
        os: protobuf::CodedOutputStream<'a>,
        is: protobuf::CodedInputStream<'a>
    }

    impl<'a> Streams<'a> {

        pub fn connect(tcp: &'a mut Tcp) -> std::io::Result<Streams<'a>> {

            let mut rpc = Streams {
                os: protobuf::CodedOutputStream::new(&mut tcp.out_stream),
                is: protobuf::CodedInputStream::new(&mut tcp.in_stream)
            };

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

            self.os.write_raw_varint32(s)?;
            m.write_to(&mut self.os)?;
            self.os.flush()
        }

        /// Receive a message
        /// TODO How to deal with possibly receiving messages of different types?
        fn recv<M>(&mut self) -> protobuf::ProtobufResult<M>
        where M: protobuf::Message {
            self.is.read_message()
        }
    }

}

mod tests;
