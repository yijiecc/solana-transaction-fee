use solana_client::nonblocking::rpc_client::RpcClient;
#[allow(deprecated)]
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};

///
/// Transfer `sols` SOL from one account to another.
///
/// * `client` - the nonblocking [`RpcClient`]
/// * `from` - the [`Keypair`] of the wallet that trasfer from
/// * `to` - the [`Pubkey`] of the wallet that transfer to
/// * `sols` - the amount of SOLs to transfer
///
pub async fn transfer_sol(
    client: &RpcClient,
    from: &Keypair,
    to: &Pubkey,
    sols: u64,
) -> Result<(), String> {
    let transfer_instruction =
        system_instruction::transfer(&from.pubkey(), to, sols * LAMPORTS_PER_SOL);
    let latest_blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(|_| "Failed to get latest blockhash")?;

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&from.pubkey()),
        &[&from],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction).await {
        Ok(_sig) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Err("Failed to send and confirm transaction".to_string())
        }
    }
}
