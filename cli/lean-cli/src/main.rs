//! LEAN CHAIN CLI
//!
//! Command-line interface for interacting with LEAN CHAIN

use clap::{Parser, Subcommand};
use lean_crypto::{KeyPair, Address};

/// LEAN CHAIN CLI
#[derive(Parser)]
#[command(name = "lean")]
#[command(about = "LEAN CHAIN Command Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Wallet operations
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },
    
    /// Transaction operations
    Tx {
        #[command(subcommand)]
        command: TxCommands,
    },
    
    /// Validator operations
    Validator {
        #[command(subcommand)]
        command: ValidatorCommands,
    },
    
    /// Query blockchain data
    Query {
        #[command(subcommand)]
        command: QueryCommands,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Generate a new wallet
    New,
    
    /// Show wallet address from private key
    Address {
        /// Private key (hex)
        #[arg(short, long)]
        key: String,
    },
    
    /// Show wallet balance
    Balance {
        /// Address to query
        address: String,
    },
}

#[derive(Subcommand)]
enum TxCommands {
    /// Send tokens
    Send {
        /// Recipient address
        #[arg(short, long)]
        to: String,
        
        /// Amount to send
        #[arg(short, long)]
        amount: u128,
        
        /// Private key
        #[arg(short, long)]
        key: String,
    },
    
    /// Query transaction
    Get {
        /// Transaction hash
        hash: String,
    },
}

#[derive(Subcommand)]
enum ValidatorCommands {
    /// Register as a validator
    Register {
        /// Stake amount
        #[arg(short, long)]
        stake: u128,
        
        /// Commission rate (0-100)
        #[arg(short, long)]
        commission: u8,
        
        /// Private key
        #[arg(short, long)]
        key: String,
    },
    
    /// Show validator info
    Info {
        /// Validator address
        address: String,
    },
    
    /// List all validators
    List,
}

#[derive(Subcommand)]
enum QueryCommands {
    /// Get block by height
    Block {
        /// Block height
        height: u64,
    },
    
    /// Get chain info
    ChainInfo,
    
    /// Get account state
    Account {
        /// Account address
        address: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Wallet { command } => handle_wallet(command)?,
        Commands::Tx { command } => handle_tx(command)?,
        Commands::Validator { command } => handle_validator(command)?,
        Commands::Query { command } => handle_query(command)?,
    }
    
    Ok(())
}

fn handle_wallet(command: WalletCommands) -> anyhow::Result<()> {
    match command {
        WalletCommands::New => {
            let keypair = KeyPair::generate();
            let address = Address::from_public_key(keypair.public_key());
            
            println!("🔑 New Wallet Generated");
            println!("=======================");
            println!("Address:     {}", address);
            println!("Public Key:  {}", keypair.public_key().to_hex());
            println!("Private Key: {}", keypair.secret_key().to_hex());
            println!();
            println!("⚠️  IMPORTANT: Save your private key securely!");
            println!("⚠️  Never share your private key with anyone!");
        }
        
        WalletCommands::Address { key } => {
            let secret_bytes = hex::decode(&key)?;
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&secret_bytes);
            
            let secret = lean_crypto::SecretKey::from_bytes(bytes);
            let keypair = KeyPair::from_secret_key(secret)?;
            let address = Address::from_public_key(keypair.public_key());
            
            println!("Address: {}", address);
        }
        
        WalletCommands::Balance { address } => {
            println!("Querying balance for: {}", address);
            println!("Balance: 0 LEAN (not connected to node)");
        }
    }
    
    Ok(())
}

fn handle_tx(command: TxCommands) -> anyhow::Result<()> {
    match command {
        TxCommands::Send { to, amount, key } => {
            println!("Sending {} LEAN to {}", amount, to);
            println!("(not connected to node - this is a demo)");
        }
        
        TxCommands::Get { hash } => {
            println!("Querying transaction: {}", hash);
            println!("(not connected to node - this is a demo)");
        }
    }
    
    Ok(())
}

fn handle_validator(command: ValidatorCommands) -> anyhow::Result<()> {
    match command {
        ValidatorCommands::Register { stake, commission, key } => {
            println!("Registering validator with stake: {} LEAN", stake);
            println!("Commission rate: {}%", commission);
            println!("(not connected to node - this is a demo)");
        }
        
        ValidatorCommands::Info { address } => {
            println!("Validator info for: {}", address);
            println!("(not connected to node - this is a demo)");
        }
        
        ValidatorCommands::List => {
            println!("Active Validators:");
            println!("(not connected to node - this is a demo)");
        }
    }
    
    Ok(())
}

fn handle_query(command: QueryCommands) -> anyhow::Result<()> {
    match command {
        QueryCommands::Block { height } => {
            println!("Querying block at height: {}", height);
            println!("(not connected to node - this is a demo)");
        }
        
        QueryCommands::ChainInfo => {
            println!("LEAN CHAIN Info");
            println!("===============");
            println!("Chain ID: 1");
            println!("Current Height: 0");
            println!("(not connected to node - this is a demo)");
        }
        
        QueryCommands::Account { address } => {
            println!("Account state for: {}", address);
            println!("(not connected to node - this is a demo)");
        }
    }
    
    Ok(())
}
