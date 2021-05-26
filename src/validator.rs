use reqwest::{ get, Response, Error };
use super::server_node::{ ServerNode };

pub struct Validator<'a> {
    server_node: ServerNode<'a>
}

impl <'a> Validator<'a> {
    async fn getAccountBalance(&self, accountNumber: &str) ->  Result<Response, Error>{
        self.server_node.get_data(&format!("/accounts/{}/balance", accountNumber)).await
    }
}