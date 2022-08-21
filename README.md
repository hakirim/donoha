# donoha

(Unofficial) Conoha API Client library.

## Example

*main.rs*

```rust
use donoha::client::APITokenRequest;
use donoha::client::APIClient;

fn main() {
    let user_name = std::env::var("DONOHA_KEY_USER_NAME").expect(format!("Please specify {}", "DONOHA_KEY_USER_NAME").as_str());
    let tenant_id = std::env::var("DONOHA_KEY_TENANT_ID").expect(format!("Please specify {}", "DONOHA_KEY_TENANT_ID").as_str());
    let password = std::env::var("DONOHA_KEY_PASSWORD").expect(format!("Please specify {}", "DONOHA_KEY_PASSWORD").as_str());
    let api_token_request = APITokenRequest::new(user_name, tenant_id.clone());
    let api_token = api_token_request.send(password).unwrap();
    let api_client = APIClient::new(api_token);
    let result = api_client.servers_text(tenant_id.clone()).unwrap();
    println!("result: {}", result);
}
```
*run*

```
export DONOHA_KEY_USER_NAME=your_api_user_name
export DONOHA_KEY_TENANT_ID=your_api_tenant_i$
export DONOHA_KEY_PASSWORD=your_password
cargo run your_project
```

*result*
```json
{"servers":[
  {"id":"12345678-xxxx-xxxx-xxxx-xxxxxxxx3d4a",
    "links":[
      {"href":"https://compute.tyo1.conoha.io/v2/1234567890abcdefghijklmnopqrstuv/servers/12345678-xxxx-xxxx-xxxx-xxxxxxxx3d4a",
        "rel":"self"},
      {"href":"https://compute.tyo1.conoha.io/1234567890xxxxxxxxxxxxxxxxxxx56f/servers/12345678-xxxx-xxxx-xxxx-xxxxxxxxxd4a",
        "rel":"bookmark"}
    ],
    "name":"111-22-111-111"}
]}
```