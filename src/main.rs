mod server_node;
mod validator;
mod primary_validator;
mod models;

use primary_validator::{ PrimaryValidator };
use server_node::ServerNodeTrait;

use futures::executor::block_on;


pub fn main() {

   let pv: PrimaryValidator =  PrimaryValidator::new("http://54.177.121.3", None);

   let future = pv.validator.get_account_balance("431f89d714dc2d748c899b14c725a52c2b5560c2b8e2bb2bb691f41a155c3a2e");

      match block_on(future) {
      Ok(resp) => {
         println!("{:?}", resp);
      }
      Err(_) => {
         println!("Errr");
      }
   };
}