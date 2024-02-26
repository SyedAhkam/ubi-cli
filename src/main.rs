use clap::{Parser, Subcommand};

pub mod commands;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Auth(commands::auth::Auth),
    List(commands::list::List),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        SubCommand::Auth(args) => commands::auth::handle(args),
        SubCommand::List(args) => commands::list::handle(args),
    }
}
