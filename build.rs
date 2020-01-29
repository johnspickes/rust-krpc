extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["src/protos/krpc.proto"],
        includes: &["src/protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("Failed protoc");
}
