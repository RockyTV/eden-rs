extern crate reqwest;
extern crate serde_json;

use reqwest::{IntoUrl, Method, RequestBuilder};
use character::Character;

#[derive(Debug)]
pub struct Eden {
    client: reqwest::Client,
    endpoint: String
}

impl Eden {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: format!("https://esi.tech.ccp.is/latest")
        }
    }

    fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder where U: std::fmt::Display {
        self.client.request(Method::Get, &format!("{endpoint}/{url}/?datasource=tranquility", endpoint=self.endpoint, url=url))
    }

    pub fn character(&self) -> Character {
        let result: Character = Default::default();
        result
    }
}

pub mod character;