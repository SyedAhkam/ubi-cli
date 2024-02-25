use serde::Deserialize;

pub const CONFIG_DIR_NAME: &str = "ubi_cli";
pub const CREDS_FILE_NAME: &str = "creds.json";

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
