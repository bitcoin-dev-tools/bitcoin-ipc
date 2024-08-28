use crate::init_capnp;
use crate::node_capnp;
use crate::proxy_capnp;

// Create Node client
pub async fn create_node_client(
    init_client: &init_capnp::init::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<node_capnp::node::Client, Box<dyn std::error::Error>> {
    let mut make_node_request = init_client.make_node_request();
    make_node_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());

    let node_client_response = make_node_request.send().promise.await?;
    println!(
        "received make_node_request response: {:?}",
        node_client_response.get()?
    );

    Ok(node_client_response.get()?.get_result()?)
}
