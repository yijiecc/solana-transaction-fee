use solana_sdk::{signature::Keypair, signer::Signer};

use crate::helper;

/// Run a test to demonstrate base fee for 2 signers
pub async fn run() -> Result<(), String> {
    let rpc_client = helper::get_rpc_client().await?;

    let wallet = Keypair::new();

    println!(
        "Created an in memory wallet with address: {}",
        wallet.pubkey()
    );
    helper::airdrop_to(&rpc_client, &wallet.pubkey(), 10).await?;

    let balance_before = rpc_client.get_balance(&wallet.pubkey()).await.unwrap();
    println!(
        "Airdropped 10 SOLs to this wallet, balance = {} lamports",
        balance_before
    );

    let mint_result = helper::create_mint(&rpc_client, &wallet).await?;
    println!("Mint token rent = {} lamports", mint_result.account_rent);

    let balance_after = rpc_client.get_balance(&wallet.pubkey()).await.unwrap();
    println!(
        "After mint token created, balance = {} lamports",
        balance_after
    );

    let base_fee = balance_before - balance_after - mint_result.account_rent;
    println!("Base fee for this transaction is {} lamports.", base_fee);
    Ok(())
}
