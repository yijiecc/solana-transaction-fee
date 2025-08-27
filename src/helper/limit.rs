use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, instruction::Instruction, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};

/// Estimate the actual compute unit that will be consumed by `instructions`
/// Additional [`ComputeBudgetInstruction`] is considered already.
pub async fn estimate_compute_unit(
    client: &RpcClient,
    instructions: &[&Instruction],
    payer: &Keypair,
) -> Result<u64, String> {
    let limit_instruction = ComputeBudgetInstruction::set_compute_unit_limit(300_000);
    let price_instruction = ComputeBudgetInstruction::set_compute_unit_price(10000);

    let latest_blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(|_| "Failed to get latest blockhash")?;

    let mut combined_instructions: Vec<Instruction> =
        instructions.iter().map(|i| (*i).clone()).collect();
    combined_instructions.insert(0, price_instruction);
    combined_instructions.insert(0, limit_instruction);

    let mut transaction =
        Transaction::new_with_payer(&combined_instructions, Some(&payer.pubkey()));
    transaction.sign(&[payer], latest_blockhash);

    let result = client
        .simulate_transaction(&transaction)
        .await
        .map_err(|_| "Failed to simulate transaction")?;

    Ok(result.value.units_consumed.unwrap())
}

/// Add priority fee instructions to `instructions`, and use `client`
/// to send and confirm the combined transaction.
pub async fn send_instruction_with_priority_fee(
    client: &RpcClient,
    instructions: &[Instruction],
    payer: &Keypair,
    cu_limit: u32,
    cu_price: u64,
) -> Result<(), String> {
    let limit_instruction = ComputeBudgetInstruction::set_compute_unit_limit(cu_limit);
    let price_instruction = ComputeBudgetInstruction::set_compute_unit_price(cu_price);

    let latest_blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(|_| "Failed to get latest blockhash")?;

    let mut combined_instructions: Vec<Instruction> =
        instructions.iter().map(|i| (*i).clone()).collect();
    combined_instructions.insert(0, price_instruction);
    combined_instructions.insert(0, limit_instruction);

    let mut transaction =
        Transaction::new_with_payer(&combined_instructions, Some(&payer.pubkey()));
    transaction.sign(&[&payer], latest_blockhash);

    client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(|_| "Failed to send and confirm transaction")?;

    Ok(())
}

/// Get recent priority fee per CU for `accounts`.
/// This functions returns the average fee per CU, not a percentile value.
pub async fn get_recent_priority_fee_per_cu(
    client: &RpcClient,
    accounts: &[Pubkey],
) -> Result<u64, String> {
    let result = client
        .get_recent_prioritization_fees(accounts)
        .await
        .map_err(|_| " Failed to get recent priority fee")?;

    if result.len() == 0 {
        println!("No recent priority fee can be found, using default value 1000");
        return Ok(1000);
    }
    //println!("recent priority fee = {:?}", &result);

    let average = result.iter().map(|f| f.prioritization_fee).sum::<u64>() / (result.len() as u64);

    Ok(average)
}
