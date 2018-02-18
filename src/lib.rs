extern crate reqwest;
extern crate serde_json;

use reqwest::{IntoUrl, Method, RequestBuilder};
use character::Character;

#[derive(Debug)]
pub struct Eden {
    client: reqwest::Client
}

impl Eden {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new()
        }
    }

    fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client.request(Method::Get, url)
    }

    pub fn character(&self) -> Character {
        let result: Character = Default::default();
        result
    }
}

pub mod character;