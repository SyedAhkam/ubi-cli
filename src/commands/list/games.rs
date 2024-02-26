use clap::Parser;

use ubi_cli::{client::UbiClient, Credentials};

#[derive(Parser)]
pub struct Games {}

pub fn handle(_args: Games) {
    let creds = Credentials::find().expect("failed to find credentials");

    let client = UbiClient::new(creds).unwrap();

    let user = client.fetch_user().unwrap();

    println!("user: {:?}", user);
}
