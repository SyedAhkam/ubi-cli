use clap::{Parser, Subcommand};

pub mod login;
pub mod status;

#[derive(Parser)]
pub struct Auth {
    #[clap(subcommand)]
    command: AuthCommand,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    Login(login::Login),
    Status(status::Status),
}

pub fn handle(args: Auth) {
    match args.command {
        AuthCommand::Login(args) => login::handle(args),
        AuthCommand::Status(args) => status::handle(args),
    }
}
