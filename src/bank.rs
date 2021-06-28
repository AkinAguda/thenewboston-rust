use super::primary_validator::PrimaryValidator;
use super::server_node::{ServerNode, ServerNodeOptions, ServerNodeTrait};
use reqwest::Error;
use super::responses::bank::config::{BankConfig};

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
        let response: Result<BankConfig, Error> =
            self.server_node.get_data("/config").await;
        response
    }
    pub async fn get_bank_primary_validator(self) -> Result<PrimaryValidator, Error> {
        let response = self.get_config().await;
        match response {
            Ok(config) => {
                let port = match config.port {
                    Some(value) => format!(":{}", value.to_string()),
                    _ => String::from(""),
                };
                let url = format!(
                    "{}://{}{}",
                    &config.primary_validator.protocol.get_str(),
                    config.primary_validator.ip_address,
                    port
                );
                Ok(PrimaryValidator::new(url, None))
            }
            Err(error) => Err(error),
        }
    }
}
