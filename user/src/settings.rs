
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    pub PK: String,
    pub client_id: String,
    pub token_url: String,
    pub client_secret: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub url: String,
}
#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}


crate::shared::configmaker!{
    server, Server, "Server Configuration";
    security,Security,"Security Configuration";   
    env, ENV, "Environment"
}