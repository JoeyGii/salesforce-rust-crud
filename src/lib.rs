use hyper::Client;
use hyper::Request;
use hyper_tls::HttpsConnector;
mod models {
    pub mod model;
}
use models::model::{AuthorizationDeserializer, KeyValue, ObjectSearchDeserializer};
use std::collections::HashMap;
use std::env;

pub struct UpdateConfig<'a> {
    pub sobject: &'a String,
    pub sobject_id: &'a String,
    pub field_value: HashMap<&'a String, &'a String>,
}
impl<'a> UpdateConfig<'a> {
    pub fn configure(
        sobject: &'a String,
        sobject_id: &'a String,
        field_value: HashMap<&'a String, &'a String>,
    ) -> UpdateConfig<'a> {
        UpdateConfig {
            sobject: sobject,
            sobject_id: sobject_id,
            field_value: field_value,
        }
    }
}
fn query_formatter(sobj: &str, name: &str) -> String {
    format!(
        "{}&sobject={sobj}&{sobj}.fields=id,name&{sobj}.limit=1",
        name
    )
}
pub async fn get_ids<'a>(
    secret: &str,
    sobj: &'a str,
    sobj_name: &'a str,
) -> Result<String, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let uri = env::var("uri").expect("Expected a valid uri in the .env file");

    let query = format!(
        "https://{}/services/data/v57.0/parameterizedSearch/?q={}",
        uri,
        &query_formatter(sobj, sobj_name)
    );

    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method("GET")
        .uri(query)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {secret}"))
        .body(hyper::Body::from(""))?;
    let response = client.request(request).await?;
    let status_code = response.status();
    println!("Search status code: {status_code}");
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();
    let result_array: Result<ObjectSearchDeserializer, _> = serde_json::from_str(&body_as_string);

    match result_array {
        Ok(result) => {
            for (i, r) in result.searchRecords.iter().enumerate() {
                let i: usize = i + 1;
                println!("Result {i}: {r:?}")
            }
        }
        Err(e) => {
            panic!("Not able to deserialize results: {}", e)
        }
    }

    if status_code.as_u16() > 204 {
        println!("{body_as_string}");
        panic!("Update failed")
    }

    Ok(body_as_string)
}

pub async fn update<'a>(
    secret: &str,
    config: UpdateConfig<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let body: KeyValue = KeyValue {
        map: config.field_value,
    };
    let crud_body: String = serde_json::to_string(&body.map).unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = env::var("uri").expect("Expected a valid uri in the .env file");
    let request = Request::builder()
        .method("PATCH")
        .uri(format!(
            "https://{}/services/data/v56.0/sobjects/{}/{}",
            uri, config.sobject, config.sobject_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {secret}"))
        .body(hyper::Body::from(crud_body))?;
    let response = client.request(request).await?;
    let status_code = response.status();
    println!("Update status code: {status_code}");

    if status_code.as_u16() > 204 {
        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("{body_as_string}");
        panic!("Update failed")
    }

    Ok(())
}
pub async fn authorize() -> Result<AuthorizationDeserializer, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = env::var("uri").expect("Please enter a valid URI into a .env file");
    let request = Request::builder()
        .method("POST")
        .uri(format!("https://{uri}/services/oauth2/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(hyper::Body::from(format_auth_request_body()))?;
    let response = client.request(request).await?;
    let status_code = response.status();
    println!("Authorization status code: {status_code}");
    if status_code.as_u16() > 200 {
        panic!("Authorization failed. Check all .env variables")
    }
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    let deserialized_string: AuthorizationDeserializer =
        serde_json::from_str(&body_as_string).unwrap();
    Ok(deserialized_string)
}
fn format_auth_request_body() -> String {
    let client_id = env::var("client_id").expect("Please enter a valid Client Id into a .env file");
    let client_secret =
        env::var("client_secret").expect("Please enter a valid Client Secret into a .env file");
    let user = env::var("username").expect("Please enter a valid username into a .env file");
    let pw = env::var("pw").expect("Please enter a valid pw into a .env file");
    let body = format!(
        "grant_type=password&client_id=
                  {client_id}
                  &client_secret=
                  {client_secret}
                  &username=
                  {user}
                  &password=
                  {pw}"
    );
    body
}
