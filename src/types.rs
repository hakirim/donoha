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
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flavors {
    pub flavors: Vec<Flavor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flavor {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub name: String,
}
