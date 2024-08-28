use crate::chain_capnp;
use crate::init_capnp;
use crate::proxy_capnp;

// Create a Chain client
pub async fn create_chain_client(
    init_client: &init_capnp::init::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<chain_capnp::chain::Client, Box<dyn std::error::Error>> {
    let mut make_chain_request = init_client.make_chain_request();
    make_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());

    let chain_client_response = make_chain_request.send().promise.await?;
    println!(
        "received chain_client response: {:?}",
        chain_client_response.get()?
    );

    Ok(chain_client_response.get()?.get_result()?)
}

// Query Chain height
pub async fn query_chain_height(
    chain_client: &chain_capnp::chain::Client,
    thread_client: &proxy_capnp::thread::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_chain_request = chain_client.get_height_request();
    new_chain_request
        .get()
        .get_context()?
        .set_thread(thread_client.clone());
    let new_chain = new_chain_request.send().promise.await?;
    println!("received chain response: {:?}", new_chain.get()?);
    Ok(())
}
