use super::server_node::{ServerNode, ServerNodeOptions, ServerNodeTrait};
use reqwest::{Error, Response};

pub struct Validator<'a> {
    pub server_node: ServerNode<'a>,
}

impl<'a> ServerNodeTrait<'a, Validator<'a>> for Validator<'a> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> Validator<'a> {
        Validator {
            server_node: ServerNode::new(url, options),
        }
    }
}

impl<'a> Validator<'a> {
    pub async fn get_account_balance(&self, account_number: &str) -> Response {
        self.server_node
            .get_data(&format!("/accounts/{}/balance", account_number))
            .await
    }
}
