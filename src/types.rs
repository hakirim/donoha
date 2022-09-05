use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Servers {
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub tenant_id: String,
}
