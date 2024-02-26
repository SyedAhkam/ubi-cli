use clap::{Parser, Subcommand};

pub mod games;

#[derive(Parser)]
pub struct List {
    #[clap(subcommand)]
    command: ListCommand,
}

#[derive(Subcommand)]
pub enum ListCommand {
    Games(games::Games),
}

pub fn handle(args: List) {
    match args.command {
        ListCommand::Games(args) => games::handle(args),
    }
}
