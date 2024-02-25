use chrono::{DateTime, Local, Utc};
use clap::Parser;

use ubi_cli::{Credentials, CONFIG_DIR_NAME, CREDS_FILE_NAME};

#[derive(Parser)]
pub struct Status {}

fn report_status(creds: Credentials) {
    println!(
        "we are logged in as '{}' ({})",
        creds.name_on_platform, creds.user_id
    );

    println!(
        "creds expiring at {}",
        DateTime::<Local>::from(creds.expiration)
    );
}

pub fn handle(_args: Status) {
    // Get creds file path
    let config_path = dirs::config_local_dir()
        .expect("failed to find config dir")
        .join(CONFIG_DIR_NAME);

    let creds_file_path = config_path.join(CREDS_FILE_NAME);

    // Try to read
    match std::fs::read_to_string(creds_file_path) {
        Ok(creds_file) => {
            match serde_json::from_str::<Credentials>(&creds_file) {
                Ok(creds) => {
                    let current_time = Utc::now();

                    if current_time > creds.expiration {
                        eprintln!("creds are expired; please login again");
                        return;
                    }

                    report_status(creds);
                }
                Err(_) => eprintln!("invalid creds file format; try logging in again"),
            };
        }
        Err(_) => eprintln!("failed to read creds file path; probably not logged in"),
    };
}
