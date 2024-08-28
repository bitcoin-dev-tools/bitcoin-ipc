pub mod chain_capnp {
    include!(concat!(env!("OUT_DIR"), "/chain_capnp.rs"));
}
pub mod common_capnp {
    include!(concat!(env!("OUT_DIR"), "/common_capnp.rs"));
}
pub mod echo_capnp {
    include!(concat!(env!("OUT_DIR"), "/echo_capnp.rs"));
}
pub mod handler_capnp {
    include!(concat!(env!("OUT_DIR"), "/handler_capnp.rs"));
}
pub mod init_capnp {
    include!(concat!(env!("OUT_DIR"), "/init_capnp.rs"));
}
pub mod node_capnp {
    include!(concat!(env!("OUT_DIR"), "/node_capnp.rs"));
}
pub mod proxy_capnp {
    include!(concat!(env!("OUT_DIR"), "/proxy_capnp.rs"));
}
pub mod wallet_capnp {
    include!(concat!(env!("OUT_DIR"), "/wallet_capnp.rs"));
}
pub mod chain;
pub mod init;
pub mod node;
pub mod wallet;
