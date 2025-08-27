use solana_client::nonblocking::rpc_client::RpcClient;
#[allow(deprecated)]
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_token::{solana_program::program_pack::Pack, state::Mint};

pub async fn create_mint(
    client: &RpcClient,
    wallet: &Keypair,
) -> Result<CreateMintFeeSpec, String> {
    let mint = Keypair::new();
    let space = Mint::LEN;

    let rent = client
        .get_minimum_balance_for_rent_exemption(space)
        .await
        .map_err(|_| "Failed to get minimum rent for space.")?;

    // after this instruction, the mint account is owned by SPL token program
    let create_account_instruction = system_instruction::create_account(
        &wallet.pubkey(), // fee payer
        &mint.pubkey(),   // target mint account
        rent,
        space as u64,
        &spl_token::ID, // account owner
    );

    // Use this instruction to update data inside the mint account
    let initialize_mint_instruction = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),   // mint pubkey
        &wallet.pubkey(), // mint authority
        Some(&wallet.pubkey()),
        2, // decimals
    )
    .map_err(|e| {
        println!("Failed to construct initialize_mint_instruction: {:?}", e);
        "Failed to construct initialize_mint_instruction"
    })?;

    let latest_blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(|_| "Failed to get latest blockhash")?;

    // Since we are creating new account, we need 2 signers
    // 1. The rent payer, which is wallet account
    // 2. The newly created account, which is used to prove we have control over this account
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction], // instructions
        Some(&wallet.pubkey()),                                     // payer
        &[wallet, &mint],                                           // signers
        latest_blockhash,
    );

    client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(|e| {
            println!("Failed to send_and_confirm_transaction: {:?}", e);
            "Failed to send_and_confirm_transaction"
        })?;

    Ok(CreateMintFeeSpec {
        signers_count: 2,
        account_rent: rent,
    })
}

#[allow(unused)]
pub struct CreateMintFeeSpec {
    pub signers_count: i32,
    pub account_rent: u64,
}
