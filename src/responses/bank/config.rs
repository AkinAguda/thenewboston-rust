use super::super::super::constants::{Hex, NodeType, Origin, Protocol, Trust, Url};
// Some of these fields have incorrect types
struct PrimaryValidatorFields<'a> {
    account_number: Hex<'a>,
    ip_address: Origin<'a>,
    node_identifier: Hex<'a>,
    port: Option<u16>,
    protocol: Protocol,
    version: &'a str,
    default_transaction_fee: u64,
    root_account_file: Url<'a>,
    root_account_file_hash: Hex<'a>,
    seed_block_identifier: Hex<'a>,
    daily_confirmation_rate: Option<u32>,
    trust: Trust<'a>,
}

pub struct BankConfigResponse<'a> {
    primary_validator: PrimaryValidatorFields<'a>,
    account_number: Hex<'a>,
    ip_address: Origin<'a>,
    node_identifier: Hex<'a>,
    port: Option<u16>,
    protocol: Protocol,
    version: &'a str,
    default_transaction_fee: u64,
    node_type: NodeType,
}
