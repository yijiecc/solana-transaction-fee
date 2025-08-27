use solana_client::nonblocking::rpc_client::RpcClient;

/// Get a default RPC client to localnet
pub async fn get_rpc_client() -> Result<RpcClient, String> {
    let rpc = RpcClient::new("http://localhost:8899".to_string());
    match rpc.get_latest_blockhash().await {
        Ok(_) => Ok(rpc),
        Err(e) => {
            println!("{:?}", e);
            Err(format!(
                "Failed to connect to localnet, please check if solana-test-validator is running."
            ))
        }
    }
}
