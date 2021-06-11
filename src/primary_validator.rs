use super::server_node::{ServerNodeOptions, ServerNodeTrait};
use super::validator::Validator;
pub struct PrimaryValidator<'a> {
    pub validator: Validator<'a>,
}

impl<'a> ServerNodeTrait<'a, PrimaryValidator<'a>> for PrimaryValidator<'a> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> PrimaryValidator<'a> {
        PrimaryValidator {
            validator: Validator::new(url, options),
        }
    }
}
