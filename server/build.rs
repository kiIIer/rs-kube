use std::io;

fn main() -> Result<(), io::Error> {
    tonic_build::compile_protos("proto/messenger.proto")
}
