mod db;
mod task;

use clap::{Args, Parser, Subcommand};
use task::Task;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add(AddArgs),
    List,
}

#[derive(Args, Debug, Clone)]
struct AddArgs {
    name: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.cmd {
        Commands::Add(args) => {
            Task::create(args.name.clone());
        }
        Commands::List => Task::list(),
    }
}
