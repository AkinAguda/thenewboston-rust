mod server_node;
mod validator;
mod primary_validator;
mod models;

use primary_validator::{ PrimaryValidator };
use server_node::ServerNodeTrait;

use futures::executor::block_on;


pub fn main() {

   let pv: PrimaryValidator =  PrimaryValidator::new("http://54.177.121.3", None);

   let future = pv.validator.get_account_balance("10b8a9c088344001bcd2d419286ff37969fb869f2c17593b902f8fe86e690097");

      match block_on(future) {
      Ok(resp) => {
         println!("{:?}", resp);
      }
      Err(_) => {
         println!("Errr");
      }
   };
}