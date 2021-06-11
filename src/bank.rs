use super::server_node::{ServerNode, ServerNodeOptions, ServerNodeTrait};
use reqwest::{Error, Response};

pub struct Bank<'a> {
    pub server_node: ServerNode<'a>,
}

impl<'a> ServerNodeTrait<'a, Bank<'a>> for Bank<'a> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> Bank<'a> {
        Bank {
            server_node: ServerNode::new(url, options),
        }
    }
}

impl<'a> Bank<'a> {
    pub async fn get_config(&self) -> Result<Response, Error> {
        self.server_node.get_data("/config").await
    }
}
