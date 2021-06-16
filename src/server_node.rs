use reqwest::{get, Error};
use serde::Deserialize;

/** The pagination options that can be sent with requests and the instantiation of a new server node class. */
pub struct PaginationOptions {
    pub limit: u32,
    pub offset: u32,
}

/** The options that are available for all server nodes. */
pub struct ServerNodeOptions {
    pub default_pagination: PaginationOptions,
}

pub struct ServerNode {
    pub url: String,
    pub options: Option<ServerNodeOptions>,
}

pub trait ServerNodeTrait<T> {
    fn new(url: String, options: Option<ServerNodeOptions>) -> T;
}

impl ServerNodeTrait<ServerNode> for ServerNode {
    fn new(url: String, options: Option<ServerNodeOptions>) -> ServerNode {
        ServerNode { url, options }
    }
}

pub trait DeserializeOwned: for<'de> Deserialize<'de> {}

impl ServerNode {
    /**
        This reaches out to an API to get a resource and parses the returned JSON.
        When called, the type T must be defnined in the return type.
    */
    pub async fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let response = get(format!("{}{}", self.url, endpoint)).await;
        match response {
            Ok(result) => {
                let parsed = result.json::<T>().await;
                match parsed {
                    Ok(parsed_data) => Ok(parsed_data),
                    Err(parse_error) => Err(parse_error),
                }
            }
            Err(response_error) => Err(response_error),
        }
    }
}
