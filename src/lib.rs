#[cfg(test)]
//use mockall::{automock, mock, predicate::*};

/// APIクライアント。実際の処理は内部の Gateway へ委ねる。
pub struct APIClient<'a> {
    gateway: &'a dyn Gateway,
}

impl<'a> APIClient<'a> {
    pub fn new(gateway: &'a dyn Gateway) -> Self {
        Self { gateway }
    }
}

/// 実際にAPIを呼び出す、もしくは、
/// テスト用のAPIレスポンスを返却する。
trait Gateway {
    fn servers(&self) -> Vec<Server>;
    fn header(&self, key: String, value: String) -> Self;
    fn body(&self, body: String) -> Self;
    fn send(&self) -> Self;
}

struct GatewayByReqwest {
    reqwest_client: reqwest::Client,
}

impl GatewayByReqwest {
    fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}
impl Gateway for GatewayByReqwest {
    fn header(&self, key: String, value: String) -> Self {
        self.reqwest_client.header(key, value);
        self
    }

    fn body(&self, body: String) -> Self {
        self.reqwest_client.body(body);
        self
    }

    fn send(&self) -> Self {
        self.reqwest_client.send()
    }
}

/// Conoha server を表す
struct Server {
    id: String,
}

/// Conoha API を利用するための Token request
pub struct APITokenRequest<'a> {
    gateway: &'a dyn Gateway,
    user_name: String,
    tenant_id: String,
}

impl<'a> APITokenRequest {
    /// Token request を送信する
    ///
    /// # Example
    ///
    /// ```
    ///
    /// struct MockAPITokenRequestGateway {}
    ///
    /// impl Gateway for MockAPITokenRequestGateway {
    /// }
    /// let token_request = APITokenRequest {
    ///     gateway : Gateway{},
    ///     user_name: "Name001",
    ///     tenant_id: "tenant001",
    /// }
    ///
    /// let result = token_request.send(String::new("password123"));
    /// ```
    pub fn send(&self, password: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.client
            .post(self.url())
            .header(reqwest::header::ACCEPT, "application/json")
            .body(self.params(password))
            .send()
    }

    fn url(&self) -> String {
        String::from("https://identity.tyo1.conoha.io/v2.0/tokens")
    }

    fn params(&self, password: String) -> String {
        format!("{{ \"auth\": {{ \"passwordCredentials\": {{ \"username\": \"{}\", \"password\": \"{}\" }}, \"tenantId\": \"{}\" }} }}", self.
                             user_name, password, self.tenant_id)
    }
}

pub struct APIToken {
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn read_token_from_env() {
        let client_builder = ClientBuilder::new();
        client_builder.http_client(reqwest::Client::new());
    }
}
