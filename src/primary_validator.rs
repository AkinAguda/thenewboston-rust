use super::validator::{ Validator };
use super::server_node::{ ServerNodeTrait, ServerNodeOptions };
pub struct PrimaryValidator<'a> {
    pub validator: Validator<'a>
}

impl<'a> ServerNodeTrait<'a, PrimaryValidator<'a>> for PrimaryValidator<'a> {
    fn new(url: &'a str, options: Option<ServerNodeOptions>) -> PrimaryValidator<'a> {
        PrimaryValidator {
            validator: Validator::new(url, options)
        }
    }
}

