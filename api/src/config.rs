use mongodb::options::ClientOptions;
use std::env;

pub struct Config {
    pub mongodb_option: ClientOptions,
}

impl Config {
    pub async fn from_env() -> Self {
        let mongodb_option = ClientOptions::parse(env::var("MONGODB_URI").unwrap())
            .await
            .unwrap();
        Self { mongodb_option }
    }
}
