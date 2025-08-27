mod base_fee;
mod helper;
mod priority_fee;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("-------------------------------------");
    base_fee::simple::run().await?;

    println!("-------------------------------------");
    base_fee::multisig::run().await?;

    println!("-------------------------------------");
    priority_fee::example::run().await?;
    Ok(())
}
