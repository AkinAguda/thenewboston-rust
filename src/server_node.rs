use reqwest::{get, Error, Response};

/** The pagination options that can be sent with requests and the instantiation of a new server node class. */
pub struct PaginationOptions {
    pub limit: u32,
    pub offset: u32,
}

/** The options that are available for all server nodes. */
pub struct ServerNodeOptions {
    pub default_pagination: PaginationOptions,
}

pub struct ServerNode<'a> {
    pub url: &'a str,
    pub options: Option<ServerNodeOptions>,
}

pub trait ServerNodeTrait<'a, T> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> T;
}

impl<'a> ServerNodeTrait<'a, ServerNode<'a>> for ServerNode<'a> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> ServerNode<'a> {
        ServerNode { url, options }
    }
}

impl<'a> ServerNode<'a> {
    pub async fn get_data(&self, endpoint: &str) -> Response {
        let body = get(format!("{}{}", self.url, endpoint)).await;
        body.expect(&format!("request to {}{} failed", self.url, endpoint))
    }
}
