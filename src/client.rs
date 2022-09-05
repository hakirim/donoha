use crate::types::{Server, Servers};
use reqwest;
use reqwest::blocking::RequestBuilder;

/// Conoha API を利用するための Token request
pub struct APITokenRequest {
    client: reqwest::blocking::Client,
    user_name: String,
    tenant_id: String,
}

impl APITokenRequest {
    pub fn new(user_name: String, tenant_id: String) -> Self {
        APITokenRequest {
            client: reqwest::blocking::Client::new(),
            user_name,
            tenant_id,
        }
    }

    pub fn send(&self, password: String) -> Result<APIToken, reqwest::Error> {
        let token_position = 15;
        let send_result = self
            .client
            .post(self.url())
            .header(reqwest::header::ACCEPT, "application/json")
            .body(self.params(password))
            .send();
        return match send_result {
            Err(e) => Err(e),
            Ok(r) => match r.text() {
                Err(e) => Err(e),
                Ok(text) => {
                    let list: Vec<&str> = text.split('"').collect();
                    let str_token: String = list[token_position].to_string();
                    Ok(APIToken { value: str_token })
                }
            },
        };
    }

    fn url(&self) -> &str {
        "https://identity.tyo1.conoha.io/v2.0/tokens"
    }

    fn params(&self, password: String) -> String {
        let ret = format!("{{ \"auth\": {{ \"passwordCredentials\": {{ \"username\": \"{}\", \"password\": \"{}\" }}, \"tenantId\": \"{}\" }} }}", self.
            user_name, password, self.tenant_id);
        String::from(ret)
    }
}

/// Conoha API を利用するための Token
pub struct APIToken {
    value: String,
}

/// Conoha API を利用するための API Client
pub struct APIClient {
    http_client: reqwest::blocking::Client,
    api_token: APIToken,
}

enum HTTPMethod {
    GET,
    POST,
    DELETE,
}

impl APIClient {
    pub fn new(api_token: APIToken) -> Self {
        APIClient {
            http_client: reqwest::blocking::Client::new(),
            api_token,
        }
    }

    fn basic_request(&self, method: HTTPMethod, url: &str) -> RequestBuilder {
        let request = match method {
            HTTPMethod::GET => self.http_client.get(url),
            HTTPMethod::POST => self.http_client.post(url),
            HTTPMethod::DELETE => self.http_client.delete(url),
        };
        request
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Auth-Token", self.api_token.value.clone())
    }

    /// サーバー一覧をJSON文字列として取得する
    pub fn servers_text(&self, tenant_id: &str) -> Option<String> {
        // doc: https://www.conoha.jp/docs/compute-get_flavors_detail.php
        let tenant_id_for_url = String::from(tenant_id);
        let result = self
            .basic_request(
                HTTPMethod::GET,
                format!(
                    "https://compute.tyo1.conoha.io/v2/{}/servers/detail",
                    tenant_id_for_url
                )
                .as_str(),
            )
            .send();
        result.unwrap().text().ok()
    }

    /// サーバー一覧を取得する
    pub fn servers(&self, tenant_id: &str) -> Option<Servers> {
        let text = self.servers_text(tenant_id);
        text.and_then(|json| Some::<Servers>(serde_json::from_str(&json).unwrap()))
    }

    pub fn shutdown(&self, server: &Server) -> bool {
        // doc: https://www.conoha.jp/docs/compute-stop_cleanly_vm.php
        let url = format!(
            "https://compute.tyo1.conoha.io/v2/{}/servers/{}/action",
            server.tenant_id, server.id
        );
        let url = url.as_str();

        let request = self.basic_request(HTTPMethod::POST, url);
        let result = request.body("{\"os-stop\": null}").send();
        match result {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    pub fn delete(&self, server: &Server) -> bool {
        // doc: https://www.conoha.jp/docs/compute-delete_vm.php
        let url = format!(
            "https://compute.tyo1.conoha.io/v2/{}/servers/{}",
            server.tenant_id, server.id
        );
        let url = url.as_str();

        let request = self.basic_request(HTTPMethod::DELETE, url);
        let result = request.body("{\"os-stop\": null}").send();
        match result {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }
}
