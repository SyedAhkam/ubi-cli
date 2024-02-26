use clap::Parser;

use ubi_cli::{client::UbiClient, Credentials};

#[derive(Parser)]
pub struct Games {}

pub fn handle(_args: Games) {
    let creds = Credentials::find().expect("failed to find credentials");

    let client = UbiClient::new(creds).unwrap();

    let games = client.fetch_games().unwrap();

    for (idx, game) in games.into_iter().enumerate() {
        println!("{}. {} [{}]", idx + 1, game.name, game.id);
    }
}
