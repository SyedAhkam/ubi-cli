use std::str::FromStr;

use anyhow::Result;
use reqwest::header;

use crate::{resources::user::User, Credentials, APP_ID};

pub struct UbiClient {
    pub creds: Credentials,
    req_client: reqwest::blocking::Client,
}

impl UbiClient {
    pub fn new(creds: Credentials) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Ubi_v1 t={}", creds.ticket))?,
        );

        headers.insert(
            header::HeaderName::from_str("Ubi-AppId")?,
            header::HeaderValue::from_str(APP_ID)?,
        );

        headers.insert(
            header::HeaderName::from_str("Ubi-SessionId")?,
            header::HeaderValue::from_str(&creds.session_id)?,
        );

        let req_client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .default_headers(headers)
            .build()?;

        Ok(Self { creds, req_client })
    }

    pub fn fetch_user(&self) -> Result<User> {
        let resp = self
            .req_client
            .get(format!(
                "https://public-ubiservices.ubi.com/v3/users/{}",
                self.creds.user_id
            ))
            .send()?;

        Ok(resp.json::<User>()?)
    }
}
