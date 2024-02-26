pub mod client;
pub mod resources;

// Re-exports for convinience
pub use resources::credentials::Credentials;

pub const CONFIG_DIR_NAME: &str = "ubi_cli";
pub const CREDS_FILE_NAME: &str = "creds.json";

// These variables are taken straight from the official ubisoft site
pub const APP_ID: &str = "314d4fef-e568-454a-ae06-43e3bece12a6";
pub const GENOME_ID: &str = "85c31714-0941-4876-a18d-2c7e9dce8d40";
