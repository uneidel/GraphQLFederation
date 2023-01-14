
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]


pub struct Security {
    pub pk: String,
    pub jwt_enabled: bool,
    pub playground: bool,
    pub requiredrole: String
   
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

shared::configmaker!{
    server, Server, "Server Configuration";
    security,Security,"Security Configuration";   
    env, ENV, "Environment"
}