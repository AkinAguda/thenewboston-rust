use super::super::super::constants::{Hex, NodeType, Origin, Protocol, Trust, Url};
use serde::{self, de};
use super::super::super::server_node::DeserializeOwned;
use serde::Deserialize;
// Some of these fields have incorrect types
#[derive(Deserialize, Debug)]
pub struct PrimaryValidatorResponse {
    pub account_number: Hex,
    pub ip_address: Origin,
    pub node_identifier: Hex,
    pub port: Option<u16>,
    pub protocol: Protocol,
    pub version: String,
    pub default_transaction_fee: u64,
    pub root_account_file: Url,
    pub root_account_file_hash: Hex,
    pub seed_block_identifier: Hex,
    pub daily_confirmation_rate: Option<u32>,
    pub trust: Trust,
}

#[derive(Deserialize, Debug)]
pub struct BankConfig {
    pub primary_validator: PrimaryValidatorResponse,
    pub account_number: Hex,
    pub ip_address: Origin,
    pub node_identifier: Hex,
    pub port: Option<u16>,
    pub protocol: Protocol,
    pub version: String,
    pub default_transaction_fee: u64,
    pub node_type: NodeType,
}

impl DeserializeOwned for BankConfig {}