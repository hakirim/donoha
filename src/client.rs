use reqwest;
use reqwest::blocking::RequestBuilder;
use crate::types::Servers;

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
        let send_result = self.client
            .post(self.url())
            .header(reqwest::header::ACCEPT, "application/json")
            .body(self.params(password))
            .send();
        return match send_result {
            Err(e) => {
                Err(e)
            }
            Ok(r) => {
                match r.text() {
                    Err(e) => {
                        Err(e)
                    }
                    Ok(text) => {
                        let list: Vec<&str> = text.split('"').collect();
                        let str_token = list[token_position].to_string();
                        Ok(APIToken { value: str_token })
                    }
                }
            }
        }
    }

    fn url(&self) -> String {
        String::from("https://identity.tyo1.conoha.io/v2.0/tokens")
    }

    fn params(&self, password: String) -> String {
        format!("{{ \"auth\": {{ \"passwordCredentials\": {{ \"username\": \"{}\", \"password\": \"{}\" }}, \"tenantId\": \"{}\" }} }}", self.
            user_name, password, self.tenant_id)
    }
}

/// Conoha API を利用するための Token
pub struct APIToken {
    value: String
}

/// Conoha API を利用するための API Client
pub struct APIClient {
    http_client: reqwest::blocking::Client,
    api_token: APIToken,
}

impl APIClient {
    pub fn new(api_token: APIToken) -> Self {
        APIClient {
            http_client: reqwest::blocking::Client::new(),
            api_token
        }
    }

    fn basic_request(&self, url: String) -> RequestBuilder {
        self.http_client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Auth-Token", &(self.api_token.value))
    }

    /// サーバー一覧をJSON文字列として取得する
    pub fn servers_text(&self, tenant_id: String) -> Option<String> {
        let url = format!("https://compute.tyo1.conoha.io/v2/{}/servers", tenant_id);
        let result = self.basic_request(url).send();
        result.unwrap().text().ok()
    }

    /// サーバー一覧をJSON文字列として取得する
    pub fn servers(&self, tenant_id: String) -> Option<Servers> {
        let text = self.servers_text(tenant_id);
        text.and_then(|json| Some::<Servers>(serde_json::from_str(&json).unwrap()))
    }
}
