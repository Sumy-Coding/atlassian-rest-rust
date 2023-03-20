pub mod page_service {
    use reqwest::Response;
    use crate::models::models::{Content, CreatePage};

    pub async fn get_page(url: &str, token: String, id: String) -> Content {
        let request_url = format!("{url}/rest/api/content/{id}");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        println!("{:?}", body); // todo - debug
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn create_page(conf_url: &str, token: &str, page: CreatePage) -> String {
        let request_url = format!("{conf_url}/rest/api/content/");
        let client = reqwest::Client::new();
        let res = client.post(&request_url)
            .json(&page)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        let created: String = res.text().await.unwrap();
        return created;
    }




}