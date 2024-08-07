use crate::init_capnp;
use crate::proxy_capnp;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::path::Path;
use tokio::net::UnixStream;
use tokio_util::compat::*;

// Setup the connection, initialize the RPC system and return init client and thread client
pub async fn setup_connection(
    path: &Path,
) -> Result<(init_capnp::init::Client, proxy_capnp::thread::Client), Box<dyn std::error::Error>> {
    let stream = UnixStream::connect(path).await?;
    let (reader, writer) = stream.into_split();
    let reader_compat = reader.compat();
    let writer_compat = writer.compat_write();

    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader_compat,
        writer_compat,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));

    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let init_client: init_capnp::init::Client =
        rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    tokio::task::spawn_local(rpc_system);

    let construct_response = init_client.construct_request().send().promise.await?;
    println!(
        "received construct response: {:?}",
        construct_response.get()?
    );

    let thread_map: proxy_capnp::thread_map::Client = construct_response.get()?.get_thread_map()?;
    let thread_request = thread_map.make_thread_request();
    let thread_response = thread_request.send().promise.await?;
    println!("received thread response: {:?}", thread_response.get()?);

    let thread_client: proxy_capnp::thread::Client = thread_response.get()?.get_result()?;

    Ok((init_client, thread_client))
}
