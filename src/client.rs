use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use reqwest::header;
use serde_json::json;

use crate::{
    resources::{game::Game, user::User},
    Credentials, APP_ID,
};

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

    pub fn fetch_games(&self) -> Result<Vec<Game>> {
        let data = json!({
            "operationName": "AllGames",
            "query": r#"
                fragment ownedGameProps on Game {
                    id,
                    spaceId,
                    name,
                }

                query AllGames {
                    viewer {
                        games {
                            nodes {
                                ...ownedGameProps
                            }
                        }
                    }
                }
            "#
        });

        let resp = self
            .req_client
            .post("https://public-ubiservices.ubi.com/v1/profiles/me/uplay/graphql")
            .json(data.as_object().unwrap())
            .send()?;

        let parsed = resp.json::<HashMap<String, serde_json::Value>>()?;
        let nodes = parsed
            .get("data")
            .unwrap()
            .get("viewer")
            .unwrap()
            .get("games")
            .unwrap()
            .get("nodes")
            .unwrap();

        Ok(serde_json::from_value(nodes.to_owned())?)
    }
}
