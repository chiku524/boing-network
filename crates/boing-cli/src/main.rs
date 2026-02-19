//! Boing CLI — init, dev, deploy for dApp development.

use clap::{CommandFactory, Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod init;
mod dev;
mod deploy;
mod metrics_register;

#[derive(Parser)]
#[command(name = "boing")]
#[command(about = "Boing SDK — build dApps on Boing Network")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// RPC URL (default: http://127.0.0.1:8545)
    #[arg(long, global = true, default_value = "http://127.0.0.1:8545")]
    rpc_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new dApp project
    Init {
        /// Project name (default: my-boing-dapp)
        name: Option<String>,
        /// Output directory (default: `./NAME` where NAME is the project name)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Start local dev chain
    Dev {
        /// RPC port
        #[arg(long, default_value = "8545")]
        port: u16,
    },
    /// Deploy contract or config to network
    Deploy {
        /// Path to contract bytecode or config
        #[arg(default_value = ".")]
        path: String,
    },
    /// Register contract for success-based incentives
    #[command(subcommand)]
    Metrics(MetricsCommands),
    /// Generate shell completion script (bash, zsh, fish, powershell)
    Completions {
        /// Shell: bash, zsh, fish, powershell, elvish
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
enum MetricsCommands {
    /// Register contract for dApp incentive tracking
    Register {
        /// Contract account ID (hex)
        #[arg(long)]
        contract: String,
        /// dApp owner account ID (hex)
        #[arg(long)]
        owner: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, output } => init::run(name, output)?,
        Commands::Dev { port } => dev::run(port).await?,
        Commands::Deploy { path } => deploy::run(&cli.rpc_url, &path).await?,
        Commands::Metrics(MetricsCommands::Register { contract, owner }) => {
            metrics_register::run(&cli.rpc_url, &contract, &owner).await?;
        }
        Commands::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "boing",
                &mut std::io::stdout(),
            );
        }
    }

    Ok(())
}
