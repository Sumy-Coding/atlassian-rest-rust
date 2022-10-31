use std::collections::HashMap;
use std::fmt::format;
use std::future::Future;
use std::os::unix::raw::ino_t;
use serde::{Deserialize, Serialize};
use base64::encode;
use reqwest::Error;
use serde_json::Value;

mod pages;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Extentions {
    // #[serde(skip_serializing_if = "String::is_empty")]
    position: Value,
}

// #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
// #[repr(u8)]
// enum PositionEnum {
//     Str = 2,
//     Numm = 3,
// }

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Content {
    id: String,
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    Type: String,
    status: String,
    title: String,
    extensions: Extentions,
    _expandable: Expandable,
    _links: ContentLinks,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Expandable {
    container: String,
    metadata: String,
    operations: String,
    children: String,
    restrictions: String,
    history: String,
    ancestors: String,
    body: String,
    version: String,
    descendants: String,
    space: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct ContentLinks {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    sself: String,
    webui: String,
    edit: String,
    tinyui: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Links {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    sself: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    next: String,
    base: String,
    context: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct ContentResponse {
    results: Vec<Content>,
    start: i8,
    limit: i8,
    size: i8,
    // #[serde(skip_serializing_if = "Option::is_none")]
    _links: Links,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreatePage {
    title: String,
    body: String,
    space_key: String,
    parent_id: String,
}

async fn create_page(page: CreatePage) -> Content {
    let mut map = HashMap::new();
    map.insert("title", "Some page");
    map.insert("space", "test45");

    let request_url= format!("http://localhost:{port}/rest/api/content",
                             port = 7201);
    let client = reqwest::Client::new();
    let res = client.post(&request_url)
        .json(&map)
        .send()
        .await?;
    let created: Content = res.json().await.unwrap();
    created
}

#[tokio::main]
async fn main() -> Result<(),   Error> {
    println!("{}", "[ *** ] Starting");
    let request_url= format!("http://localhost:{port}/rest/api/content",
                               port = 7201);
    let token = encode(b"admin:admin");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("Authorization", format!("Basic {token}"))
        .send()
        .await?;

    let pages: ContentResponse = res.json().await?;

    for page in pages.results {
        let get_page = format!("http://localhost:{port}/rest/api/content/{id}", port=7201, id=page.id);
        let page_res = client.get(get_page)
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await?;
        let page: Content = page_res.json().await?;
        println!("{:?}", page)
    }

    Ok(())
}
