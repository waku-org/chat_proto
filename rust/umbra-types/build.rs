extern crate protoc_rust;

use std::env;
use std::path::PathBuf;

fn get_proto_dir() -> PathBuf {
    println!(
        "Manifest dir: {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    // protos are stored in ../../proto/
    let mut proto_root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    proto_root_dir.pop();
    proto_root_dir.pop();
    proto_root_dir.push("proto");
    proto_root_dir.push("umbra");

    println!("proto_dir: {:?}", proto_root_dir);
    proto_root_dir
}

fn main() {
    let mut config = prost_build::Config::new();

    // Create mod file to maintain import hierarchy
    config.include_file("mod.rs");

    config
        .compile_protos(
            &[
                "envelope.proto",
                "common_frames.proto",
                "conversations/private_v1.proto",
                "encryption.proto",
                "inbox.proto",
                "invite.proto",
                "reliability.proto"
            ],
            // set proto_path
            &[get_proto_dir().to_str().unwrap()],
        )
        .unwrap();
}
