use clap::{Parser, Subcommand};

pub mod login;

#[derive(Parser)]
pub struct Auth {
    #[clap(subcommand)]
    command: AuthCommand,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    Login(login::Login),
}

pub fn handle(args: Auth) {
    match args.command {
        AuthCommand::Login(args) => login::handle(args),
    }
}
