use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct KeyValue<'a> {
    pub map: HashMap<&'a String, &'a String>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectSearchDeserializer {
    pub searchRecords: Vec<SearchRecords>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRecords {
    attributes: Attributes,
    Id: String,
    Name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Attributes {
    r#type: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationDeserializer {
    pub access_token: String,
    instance_url: String,
    id: String,
    token_type: String,
    signature: String,
    issued_at: String,
}
