use hyper::Client;
use hyper::Request;
use hyper_tls::HttpsConnector;
pub mod models {
    pub mod arg_model;
    pub mod error_handler;
    pub mod model;
    pub mod state_model;
}
pub mod ui {
    pub mod ui_render_handler;
    pub mod ui_text;
}
pub mod app_inputs;
use models::arg_model::Args;
use models::model::SearchRecords;
use models::model::{AuthorizationDeserializer, KeyValue, ObjectSearchDeserializer};
use models::state_model::App;
use std::collections::HashMap;
use std::env;
use std::{error::Error, ffi::OsString, io};
mod utils;
use utils::{
    CLIENT_ID_ERROR_MSG, CLIENT_SECRET_ERROR_MSG, PASSWORD_ERROR_MSG, URI_ERROR_MSG,
    USERNAME_ERROR_MSG,
};

use crate::models::state_model::Message;

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

pub async fn configure_updates(
    a: Args,
    token: &String,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let fields_copied = a.fields;
    let fields_copied = fields_copied
        .chunks_exact(2)
        .map(|chunk| (&chunk[0], &chunk[1]))
        .collect::<HashMap<_, _>>();
    let config = UpdateConfig::configure(&a.sobj, &a.id, fields_copied);
    update(&token, config, app).await?;
    Ok(())
}

pub async fn update<'a>(
    secret: &str,
    config: UpdateConfig<'a>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let body: KeyValue = KeyValue {
        map: config.field_value,
    };
    let crud_body: String = serde_json::to_string(&body.map).unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = env::var("uri").expect(URI_ERROR_MSG);
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
    if status_code.as_u16() > 204 {
        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("{body_as_string}");
        panic!("Update failed")
    } else {
        let messages: Vec<Message> = vec![
            Message {
                body: format!("Update Success"),
            },
            Message {
                body: format!("{:?} on {}", body.map, config.sobject),
            },
        ];

        app.messages = messages;
    }
    Ok(())
}

fn query_formatter(sobj: &str, name: &str) -> String {
    format!(
        "{}&sobject={sobj}&{sobj}.fields=id,name&{sobj}.limit=10",
        name
    )
}
pub async fn describe_fields(
    token: &str,
    sobj: &str,
    id: &str,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let uri = env::var("uri").expect(URI_ERROR_MSG);

    let query = format!(
        "https://{}/services/data/v57.0/sobjects/{}/{}",
        &uri, sobj, id
    );

    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method("GET")
        .uri(query)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(hyper::Body::from(""))?;
    let response = client.request(request).await?;

    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    let body_as_string: Vec<&str> = body_as_string.split(",").collect();
    let mut map: HashMap<&str, &str> = HashMap::new();
    let _i: Vec<_> = body_as_string
        .iter()
        .map(|kv| {
            let k: Vec<&str> = kv.split(":").collect();
            if k.len() > 1 {
                map.insert(k[0], k[1]);
            }
        })
        .collect();

    let mut messages: Vec<Message> = Vec::new();
    let _i: Vec<_> = map
        .keys()
        .map(|k| {
            let message = Message {
                body: k.to_string().replace('"', ""),
            };
            messages.push(message);
        })
        .collect();

    Ok(messages)
}
pub async fn describe_filtered_fields(
    token: &str,
    sobj: &str,
    id: &str,
    field_letter: &str,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let field_filter: char = field_letter.chars().next().unwrap();
    let https = HttpsConnector::new();
    let uri = env::var("uri").expect(URI_ERROR_MSG);

    let query = format!(
        "https://{}/services/data/v57.0/sobjects/{}/{}",
        &uri, sobj, id
    );

    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method("GET")
        .uri(query)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(hyper::Body::from(""))?;
    let response = client.request(request).await?;

    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    let body_as_string: Vec<&str> = body_as_string.split(",").collect();
    let mut map: HashMap<&str, &str> = HashMap::new();
    let _i: Vec<_> = body_as_string
        .iter()
        .map(|kv| {
            let k: Vec<&str> = kv.split(":").collect();
            if k.len() > 1 {
                map.insert(k[0], k[1]);
            }
        })
        .collect();

    let mut messages: Vec<Message> = Vec::new();
    let _i: Vec<_> = map
        .keys()
        .map(|k| {
            if k.chars().nth(1).unwrap() == field_filter {
                let message = Message {
                    body: k.to_string().replace('"', ""),
                };
                messages.push(message);
            }
        })
        .collect();

    Ok(messages)
}

pub async fn get_ids_cli<'a>(
    secret: &str,
    sobj: &'a str,
    sobj_name: &'a str,
) -> Result<Vec<SearchRecords>, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let uri = env::var("uri").expect(URI_ERROR_MSG);

    let query = format!(
        "https://{}/services/data/v57.0/parameterizedSearch/?q={}",
        &uri,
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
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_as_string = String::from_utf8(body_bytes.to_vec()).unwrap();
    let result_array: Result<ObjectSearchDeserializer, _> = serde_json::from_str(&body_as_string);

    match result_array {
        Ok(result) => {
            return Ok(result.searchRecords);
        }
        Err(e) => {
            panic!("Not able to deserialize results: {}", e)
        }
    }
}

pub async fn authorize() -> Result<AuthorizationDeserializer, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = env::var("uri").expect(URI_ERROR_MSG);
    let request = Request::builder()
        .method("POST")
        .uri(format!("https://{uri}/services/oauth2/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(hyper::Body::from(format_auth_request_body()))?;
    let response = client.request(request).await?;
    let status_code = response.status();
    // println!("Authorization status code: {status_code}");
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
    let client_id = env::var("client_id").expect(CLIENT_ID_ERROR_MSG);
    let client_secret = env::var("client_secret").expect(CLIENT_SECRET_ERROR_MSG);
    let user = env::var("username").expect(USERNAME_ERROR_MSG);
    let pw = env::var("pw").expect(PASSWORD_ERROR_MSG);
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

pub fn export_fields(fields: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg();
    let mut wtr = csv::Writer::from_path(file_path)?;
    wtr.write_record(fields)?;
    wtr.flush()?;
    Ok(())
}
/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> OsString {
    let path: OsString = "/Users/joegillick/fields.csv".to_string().into();
    path
}
