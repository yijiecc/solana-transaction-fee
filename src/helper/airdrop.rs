use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

/// Airdrop `sols` SOL to `address`
///
/// * `client` -  a nonblocking [`RpcClient`]
/// * `address` - the [`Pubkey`] of some wallet
/// * `sols` - the amount of SOLs to airdrop
///
pub async fn airdrop_to(client: &RpcClient, address: &Pubkey, sols: u64) -> Result<(), String> {
    let latest_block_hash = client
        .get_latest_blockhash()
        .await
        .map_err(|_| "Failed to get latest blockhash, please check the network")?;

    let airdrop_signature = client
        .request_airdrop_with_blockhash(address, sols * LAMPORTS_PER_SOL, &latest_block_hash)
        .await
        .map_err(|_| "")?;

    loop {
        let confirmed = client
            .confirm_transaction(&airdrop_signature)
            .await
            .map_err(|_| "Failed to confirm transaction")?;

        if confirmed {
            break;
        }
    }

    Ok(())
}
