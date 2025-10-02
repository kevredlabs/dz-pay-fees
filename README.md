# dz-pay-fees

Small command-line utility to interact with the deposit account (PDA) of a revenue distribution program on Solana.

## Installation
In the repository:

```bash
cargo build --release
```

The binary is then located in `target/release/dz-pay-fees`.

## Add binary to PATH
```bash
cargo install --path .
```

## Usage
The CLI exposes three subcommands: `pda`, `balance` and `send`.

Required global arguments:
- `--rpc-url`: RPC URL (ex: `https://api.mainnet-beta.solana.com`)
- `--validator-identity`: public key (base58) of the validator identity

### Display the Deposit PDA address
```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  pda
```

### Check the Deposit PDA balance
```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  balance
```

### Send SOL to the Deposit PDA
`send` takes a SOL amount (positional) and requires `--payer` (path to JSON keypair).

```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  --payer /path/to/payer.json \
  send 0.5
```

## Technical details
- The Deposit PDA is derived via the seed "solana_validator_deposit" and the validator identity, with the `program_id` defined in `REVENUE_DISTRIBUTION_PROGRAM_ID`.
- Transactions are sent via `RpcClient::send_and_confirm_transaction`.
- The `transfer` instructions come from the `solana-system-interface` crate.

## Troubleshooting
- Insufficient balance: fund the payer account.
- Invalid/unavailable RPC URL: check `--rpc-url`.
- Crate version errors: run `cargo update` or delete/regenerate `Cargo.lock` if necessary (for an application binary, `Cargo.lock` is normally committed).

## Security
- Never commit secrets/private keys. Keep the keypair file (`--payer`) outside the repository and protect permissions.
