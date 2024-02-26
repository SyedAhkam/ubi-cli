use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub name_on_platform: String,
    pub country: String,
    pub date_of_birth: String,
    pub email: String,
    // Just did the bare minimum imp ones right now,
    // a lot of other fields also exist
}
