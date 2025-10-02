use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;

const REVENUE_DISTRIBUTION_PROGRAM_ID: &str = "dzrevZC94tBLwuHw1dyynZxaXTWyp7yocsinyEVPtt4";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// RPC URL (ex: https://api.mainnet-beta.solana.com)
    #[arg(long)]
    rpc_url: String,

    /// Validator identity pubkey (base58)
    #[arg(long)]
    validator_identity: String,

    /// Payer keypair JSON (required only for `send`)
    #[arg(long)]
    payer: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display the deposit account (PDA) balance
    Balance,
    /// Send an amount in SOL to the deposit account (PDA)
    Send {
        /// Amount to send in SOL (ex: 0.5)
        amount: f64,
    },
    /// Display only the Deposit PDA address
    Pda,
}

fn derive_deposit_pda(validator_identity: &Pubkey) -> Pubkey {
    let program_id = REVENUE_DISTRIBUTION_PROGRAM_ID.parse::<Pubkey>().unwrap();
    let (pda, _bump) = Pubkey::find_program_address(
        &[b"solana_validator_deposit", validator_identity.as_ref()],
        &program_id,
    );
    pda
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let rpc = RpcClient::new_with_commitment(cli.rpc_url.clone(), CommitmentConfig::confirmed());
    let validator_identity = cli.validator_identity.parse::<Pubkey>()?;
    let deposit_pda = derive_deposit_pda(&validator_identity);

    match cli.command {
        Commands::Pda => {
            println!("Deposit PDA: {deposit_pda}");
        }
        Commands::Balance => {
            let lamports = rpc.get_balance(&deposit_pda)?;
            let sol = lamports as f64 / 1e9;
            println!("Deposit PDA: {deposit_pda}");
            println!("Balance     : {lamports} lamports (~{sol} SOL)");
        }
        Commands::Send { amount } => {
            let payer_path = cli.payer.ok_or(anyhow!("--payer is required for send"))?;
            let payer = read_keypair_file(&payer_path)
                .map_err(|e| anyhow!("Unable to read keypair: {e}"))?;

            let lamports = (amount * 1e9).round() as u64;
            let ix = system_instruction::transfer(&payer.pubkey(), &deposit_pda, lamports);
            let msg = Message::new(&[ix], Some(&payer.pubkey()));
            let blockhash = rpc.get_latest_blockhash()?;
            let tx = Transaction::new(&[&payer], msg, blockhash);
            let sig = rpc.send_and_confirm_transaction(&tx)?;
            println!("✅ {amount} SOL sent to deposit {deposit_pda}");
            println!("Signature: {sig}");
        }
    }

    Ok(())
}