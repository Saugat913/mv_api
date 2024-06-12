use reqwest::Client;
use reqwest::Error;

pub struct GraphQLClient {
    client: Client,
    url: String,
}

impl GraphQLClient {
    pub fn new(url: &str) -> GraphQLClient {
        GraphQLClient {
            client: Client::new(),
            url: url.to_string(),
        }
    }

    pub async fn send_query(&self, query: &str) -> Result<serde_json::Value, Error> {
        let body = serde_json::json!({"query":query});

        let response = self.client.post(&self.url).json(&body).send().await?;

        response.json().await
    }
}
