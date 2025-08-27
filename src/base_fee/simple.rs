use solana_sdk::{native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer};

use crate::helper::{self, airdrop_to};

/// Run a test to demonstrate base fee of an ordinary transaction
pub async fn run() -> Result<(), String> {
    let rpc_client = helper::get_rpc_client().await?;

    let wallet = Keypair::new();

    println!(
        "Created an in memory wallet with address: {}",
        wallet.pubkey()
    );
    airdrop_to(&rpc_client, &wallet.pubkey(), 10).await?;

    let balance_before = rpc_client.get_balance(&wallet.pubkey()).await.unwrap();
    println!(
        "Airdropped 10 SOLs to this wallet, balance = {} lamports",
        balance_before
    );

    let receiver = Keypair::new();
    helper::transfer_sol(&rpc_client, &wallet, &receiver.pubkey(), 1).await?;
    println!("Transferred 1 SOL to another account.");

    let balance_after = rpc_client.get_balance(&wallet.pubkey()).await.unwrap();
    println!("After transfer, balance = {} lamports", balance_after);

    let base_fee = balance_before - balance_after - 1 * LAMPORTS_PER_SOL;
    println!("Base fee for this transaction is {} lamports.", base_fee);
    Ok(())
}
