use figment::{
    providers::{Env, Format, Json, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Oauth2 {
    pub server_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: std::vec::Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub oauth2: Oauth2,
}

pub fn get_config() -> Config {
    let config: Config = match Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("SCHOOLOS_"))
        .join(Json::file("config.json"))
        .extract()
    {
        Ok(r) => r,
        Err(e) => {
            //println!("Error getting config: {}", e);
            std::process::exit(1);
        }
    };
    config
}
