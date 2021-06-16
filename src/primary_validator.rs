use super::server_node::{ServerNodeOptions, ServerNodeTrait};
use super::validator::Validator;
pub struct PrimaryValidator {
    pub validator: Validator,
}

impl ServerNodeTrait<PrimaryValidator> for PrimaryValidator {
    fn new(url: String, options: Option<ServerNodeOptions>) -> PrimaryValidator {
        PrimaryValidator {
            validator: Validator::new(url, options),
        }
    }
}
