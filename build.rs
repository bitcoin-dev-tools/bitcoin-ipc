use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(
        env::var("OUT_DIR").expect("OUT_DIR was not defined by the cargo environment!"),
    );
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/chain.capnp")
        .file("schema/common.capnp")
        .file("schema/echo.capnp")
        .file("schema/init.capnp")
        .file("schema/handler.capnp")
        .file("schema/node.capnp")
        .file("schema/proxy.capnp")
        .file("schema/wallet.capnp")
        .output_path(out_path)
        .run()
        .expect("schema compiler command failed to run");

    println!("cargo:rerun-if-changed=schema/chain.capnp");
    println!("cargo:rerun-if-changed=schema/common.capnp");
    println!("cargo:rerun-if-changed=schema/echo.capnp");
    println!("cargo:rerun-if-changed=schema/init.capnp");
    println!("cargo:rerun-if-changed=schema/handler.capnp");
    println!("cargo:rerun-if-changed=schema/node.capnp");
    println!("cargo:rerun-if-changed=schema/proxy.capnp");
    println!("cargo:rerun-if-changed=schema/wallet.capnp");
}
