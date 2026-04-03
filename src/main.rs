use clap::{Parser, Subcommand};
use std::process::ExitCode;
use stock_trek::verification::verify::verify;

#[derive(Debug, Parser)]
#[command(
    name = "stock-trek",
    version = "1.0",
    arg_required_else_help = true,
    about = "Stock-Trek SDK for writing code to use on https://stock-trek.com"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Verify a file is runnable on https://stock-trek.com")]
    Verify {
        #[arg(short, long)]
        file: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::Verify { file } => match verify(file) {
            Err(e) => {
                e.errors.iter().for_each(|error| println!("{}", error));
                ExitCode::from(e.exit_code)
            }
            Ok(..) => {
                println!("This code is supported for use with stock-trek.com, happy signalling!");
                ExitCode::SUCCESS
            }
        },
    }
}
