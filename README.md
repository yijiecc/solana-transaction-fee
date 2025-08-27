A sample project to demonstrate transaction fees in Solana network.

This project is written in Rust, using `solana-sdk` and `spl-token`.

## Build & Run

To run this project, you have to run a local validator using `solana-test-validator`, and just use cargo:

```bash
cargo run
```

## Base fee

The `base fee` for a transaction depends on the number of signature used. You can find a single signature transaction in [simple.rs](./src/base_fee/simple.rs), and transactions with 2 signatures in [multisig.rs](./src/base_fee/multisig.rs).

## Priority fee

Priority fee is not fixed. It depends on the compute unit consumed and compute unit price estimated.

You can find the process of estimate compute unit price in [limit.rs](./src/helper/limit.rs). In localnet environment, the estimated unit price may always be 0, you can adjust the `estimated_price_per_cu` to a fixed value and run it several times to see the effect.

```rust
helper::send_instruction_with_priority_fee(
    &rpc_client,
    &[transfer_instruction],
    &wallet,
    estimated_cu as u32,
    estimated_price_per_cu,   // change it to 1000000 to see a valid value
)
```
