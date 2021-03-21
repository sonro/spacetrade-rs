use reqwest::header;
use std::{collections::HashMap, str::FromStr, time::Duration};

const API_URL: &str = "https://api.spacetraders.io";
const REQUEST_TIMEOUT: u64 = 5;
const ACTIVE_STATUS: &str = "spacetraders is currently online and available to play";

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl Default for Client {
    fn default() -> Self {
        let client = create_request_client();
        let base_url = reqwest::Url::from_str(API_URL).expect("create Client base_url");
        Self { client, base_url }
    }
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        let client = create_request_client();
        let base_url = reqwest::Url::from_str(base_url).expect("create Client base_url");
        Self { client, base_url }
    }

    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        let client = create_reqwest_client_with_token(token);
        let base_url = reqwest::Url::from_str(base_url).expect("create Client base_url");
        Self { client, base_url }
    }

    pub fn add_token(&mut self, token: &str) -> &mut Self {
        self.client = create_reqwest_client_with_token(token);
        self
    }

    pub async fn is_server_active(&self) -> bool {
        let url = self
            .base_url
            .join("/game/status")
            .expect("join path to base_url");

        if let Ok(resp) = self.client.get(url).send().await {
            if let Ok(data) = resp.json::<HashMap<String, String>>().await {
                if let Some(status) = data.get("status") {
                    return status == ACTIVE_STATUS;
                }
            }
        }

        false
    }
}

fn create_request_client() -> reqwest::Client {
    create_request_client_builder()
        .build()
        .expect("build reqwest client")
}

fn create_reqwest_client_with_token(token: &str) -> reqwest::Client {
    let headers = create_auth_headers(token);

    create_request_client_builder()
        .default_headers(headers)
        .build()
        .expect("build reqwest client")
}

fn create_request_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder().timeout(Duration::new(REQUEST_TIMEOUT, 0))
}

fn create_auth_headers(token: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    let mut auth_value =
        header::HeaderValue::from_str(token).expect("create http auth header using token");

    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn server_status_active() {
        let mock_server = block_on(MockServer::start());
        let expected_body = format!(r#"{{"status": "{}"}}"#, ACTIVE_STATUS);
        let expected_response =
            ResponseTemplate::new(200).set_body_raw(expected_body.as_bytes(), "application/json");
        block_on(
            Mock::given(path("/game/status"))
                .and(method("GET"))
                .respond_with(expected_response)
                .expect(1)
                .mount(&mock_server),
        );
        let client = Client::new(&mock_server.uri());

        let status = block_on(client.is_server_active());

        assert_eq!(true, status);
    }

    #[test]
    #[cfg(feature = "fulltest")]
    fn server_status_no_server() {
        let client = Client::new("http://1.2.3.4/");

        let status = block_on(client.is_server_active());

        assert_eq!(false, status);
    }
}
