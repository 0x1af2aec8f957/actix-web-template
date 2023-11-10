pub mod models;

use mongodb::{bson::doc, options::{IndexOptions, ClientOptions}, Client, Collection, IndexModel, ServerInfo, error};

pub const DB_NAME: &str = env!("CARGO_PKG_NAME"); // 数据库名称

#[derive(Default)]
pub struct Uri {
    pub host: String,
    pub port: isize,
    pub user: String,
    pub password: String,
}

impl Uri {
    pub fn new(host: Option<String>, port: Option<isize>) -> Self {
        Self {
            host: host.unwrap_or("127.0.0.1".to_owned()),
            port: port.unwrap_or(27017),
            user: String::new(),
            password: String::new(),
        }
    }

    pub async fn connect(&self) -> Client {
        let uri = format!(
            "{user}:{password}@{host}:{port}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port
        );

        Client::with_uri_str(format!("mongodb://{uri}")).await.expect("failed to connect by mongodb")
    }
}