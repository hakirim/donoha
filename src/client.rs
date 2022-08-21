use reqwest;

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
        let send_result = self.client
            .post(self.url())
            .header(reqwest::header::ACCEPT, "application/json")
            .body(self.params(password))
            .send();
        match send_result {
            Err(e) => {
                return Err(e);
            }
            Ok(r) => {
                match r.text() {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(text) => {
                        let list: Vec<&str> = text.split('"').collect();
                        let str_token = list[15].to_string();
                        return Ok(APIToken{ value: str_token})
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

    /// サーバー一覧をJSON文字列として取得する
    pub fn servers_text(&self, tenant_id: String) -> Option<String> {
        let result = self.http_client
            .get(format!("https://compute.tyo1.conoha.io/v2/{}/servers", tenant_id))
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Auth-Token", &(self.api_token.value))
            .send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Some(response.text().unwrap())
                } else {
                    None
                }
            }
            Err(_err) => {
                None
            }
        }


    }
}
