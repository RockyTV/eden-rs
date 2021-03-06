extern crate reqwest;
extern crate serde_json;

#[derive(Debug, Default)]
pub struct Character {
    pub alliance_id: u64,
    pub ancestry_id: u64,
    pub birthday: String,
    pub bloodline_id: u64,
    pub corporation_id: u64,
    pub description: String,
    pub gender: String,
    pub name: String,
    pub security_status: f64
}

impl Character {
    pub fn get_public_information(&self, client: &super::Eden, id: u32) -> Result<Self, String> {
        let response = client.get(&format!("characters/{character_id}", character_id=id)).send();

        match response {
            Ok(mut data) => {
                if data.status().is_success() {
                    match data.json::<serde_json::Value>() {
                        Ok(c) => {
                            Ok(Character {
                                alliance_id: match c["alliance_id"].as_u64() {
                                    Some(id) => id,
                                    None => Default::default()
                                },
                                ancestry_id: c["ancestry_id"].as_u64().unwrap(),
                                birthday: c["birthday"].as_str().unwrap().to_string(),
                                bloodline_id: c["bloodline_id"].as_u64().unwrap(),
                                corporation_id: c["corporation_id"].as_u64().unwrap(),
                                description: c["description"].as_str().unwrap().to_string(),
                                gender: c["gender"].as_str().unwrap().to_string(),
                                name: c["name"].as_str().unwrap().to_string(),
                                security_status: c["security_status"].as_f64().unwrap(),
                            })
                        },
                        Err(_) => Err(format!("error parsing json"))
                    }
                } else if data.status().is_server_error() || data.status().is_client_error() {
                    match data.json::<serde_json::Value>() {
                        Ok(err) => Err(err["error"].as_str().unwrap().to_string()),
                        Err(_) => Err(format!("error parsing json, status: {:?}", data.status()))
                    }
                } else {
                    Err(format!("something bad happened, status: {:?}", data.status()))
                }
            },
            Err(_) => Err(format!("something bad happened"))
        }
    }
}
