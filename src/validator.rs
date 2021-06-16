use super::server_node::{ServerNode, ServerNodeOptions, ServerNodeTrait};
use reqwest::{Error, Response};

pub struct Validator {
    pub server_node: ServerNode,
}

impl ServerNodeTrait<Validator> for Validator {
    fn new(url: String, options: Option<ServerNodeOptions>) -> Validator {
        Validator {
            server_node: ServerNode::new(url, options),
        }
    }
}

impl Validator {
    // pub async fn get_account_balance(&self, account_number: &str) -> Result<Response, Error> {
    //     self.server_node
    //         .get_data(&format!("/accounts/{}/balance", account_number))
    //         .await
    // }
}
