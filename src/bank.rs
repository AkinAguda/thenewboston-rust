use super::primary_validator::PrimaryValidator;
use super::server_node::{ServerNode, ServerNodeOptions, ServerNodeTrait};
use reqwest::{Error, Response};

use super::constants::{NodeType, Protocol};
use super::responses::bank::config::{BankConfig, BankConfigResponse};

pub struct Bank {
    pub server_node: ServerNode,
}

impl ServerNodeTrait<Bank> for Bank {
    fn new(url: String, options: Option<ServerNodeOptions>) -> Bank {
        Bank {
            server_node: ServerNode::new(url, options),
        }
    }
}

impl Bank {
    pub async fn get_config(self) -> Result<BankConfig, Error> {
        let response: Result<BankConfigResponse, Error> =
            self.server_node.get_data("/config").await;

        match response {
            Ok(result) => {
                let bank_config = BankConfig {
                    primary_validator: result.primary_validator,
                    account_number: result.account_number,
                    ip_address: result.ip_address,
                    node_identifier: result.node_identifier,
                    port: result.port,
                    protocol: Protocol::Http,
                    version: result.version,
                    default_transaction_fee: result.default_transaction_fee,
                    node_type: NodeType::Bank,
                };
                Ok(bank_config)
            }
            Err(error) => Err(error),
        }
    }
    pub async fn get_bank_primary_validator(self) -> Result<PrimaryValidator, Error> {
        let response = self.get_config().await;
        match response {
            Ok(config) => {
                let port = match config.port {
                    Some(value) => value.to_string(),
                    _ => String::from(""),
                };
                let url = format!(
                    "{}://{}{}",
                    // &config.primary_validator.protocol,
                    "http",
                    config.primary_validator.ip_address,
                    port
                );
                Ok(PrimaryValidator::new(url, None))
            }
            Err(error) => Err(error),
        }
    }
}
