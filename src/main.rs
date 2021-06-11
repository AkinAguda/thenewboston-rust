mod models;
mod primary_validator;
mod server_node;
mod validator;

use primary_validator::PrimaryValidator;
use server_node::ServerNodeTrait;

use tokio::runtime::Runtime;

pub fn main() {
    let rt = Runtime::new().unwrap();

    // Fix this by creating a new bank instead and get the primary validator of that bank then instantiate primary validator to that
    let pv: PrimaryValidator = PrimaryValidator::new("http://54.177.121.3", None);

    let future = pv
        .validator
        .get_account_balance("10b8a9c088344001bcd2d419286ff37969fb869f2c17593b902f8fe86e690097");

    match rt.block_on(future) {
        Ok(resp) => {
            println!("{:?}", resp);
        }
        Err(_) => {
            println!("Errr");
        }
    };
}
