use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(
        env::var("OUT_DIR").expect("OUT_DIR was not defined by the cargo environment!"),
    );
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/mining.capnp")
        .file("schema/proxy.capnp")
        .output_path(out_path)
        .run()
        .expect("schema compiler command failed to run");

    println!("cargo:rerun-if-changed=schema/mining.capnp");
    println!("cargo:rerun-if-changed=schema/proxy.capnp");
}
