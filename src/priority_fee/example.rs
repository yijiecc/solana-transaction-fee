#[allow(deprecated)]
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, system_instruction,
};

use crate::helper;

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

    let receiver = Keypair::new();

    let transfer_instruction =
        system_instruction::transfer(&wallet.pubkey(), &receiver.pubkey(), 1 * LAMPORTS_PER_SOL);
    let estimated_cu =
        helper::estimate_compute_unit(&rpc_client, &[&transfer_instruction], &wallet).await?;
    let estimated_price_per_cu =
        helper::get_recent_priority_fee_per_cu(&rpc_client, &[wallet.pubkey(), receiver.pubkey()])
            .await?;
    println!(
        "compute unit limit = {}, compute unit price = {}",
        estimated_cu, estimated_price_per_cu
    );

    helper::send_instruction_with_priority_fee(
        &rpc_client,
        &[transfer_instruction],
        &wallet,
        estimated_cu as u32,
        estimated_price_per_cu,
    )
    .await?;
    println!("Transferred 1 SOL to another account, with priority fee paid.");

    let balance_after = rpc_client.get_balance(&wallet.pubkey()).await.unwrap();
    println!("After transfer, balance = {} lamports", balance_after);

    let total_fee = balance_before - balance_after - 1 * LAMPORTS_PER_SOL;
    println!("Total fee for this transaction is {} lamports.", total_fee);
    println!("Base fee for this transaction is 5000 lamports.");
    println!(
        "Priority fee for this transaction is {} lamports.",
        total_fee - 5000
    );
    Ok(())
}
