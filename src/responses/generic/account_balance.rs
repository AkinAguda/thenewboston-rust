use serde::Deserialize;

use super::super::super::server_node::DeserializeOwned;

#[derive(Deserialize, Debug)]
pub struct AccountBalanceResponse {
    pub balance: Option<u64>,
}

impl DeserializeOwned for AccountBalanceResponse {}
