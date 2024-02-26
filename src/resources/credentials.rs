use anyhow::Result;
use serde::Deserialize;

use crate::{CONFIG_DIR_NAME, CREDS_FILE_NAME};

// NOTE: Ommited some useless fields from here
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub ticket: String,
    pub two_factor_authentication_ticket: Option<String>,
    pub profile_id: String,
    pub user_id: String,
    pub name_on_platform: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub space_id: String,
    pub session_id: String,
    pub session_key: String,
    pub remember_me_ticket: String,
    pub remember_device_ticket: String,
    pub email: String,
}

impl Credentials {
    pub fn find() -> Result<Self> {
        let config_path = dirs::config_local_dir()
            .expect("failed to find config dir")
            .join(CONFIG_DIR_NAME);

        let creds_file_path = config_path.join(CREDS_FILE_NAME);

        let creds_file_contents = std::fs::read_to_string(creds_file_path)?;

        Ok(serde_json::from_str(&creds_file_contents)?)
    }
}
