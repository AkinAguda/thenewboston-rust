mod server_node;
mod validator;
mod primary_validator;
mod models;

use primary_validator::{ PrimaryValidator };
use server_node::ServerNodeTrait;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub fn main() {
   let pv: PrimaryValidator =  PrimaryValidator::new("http://54.177.121.3", None);

   let resp = pv.validator.get_account_balance("431f89d714dc2d748c899b14c725a52c2b5560c2b8e2bb2bb691f41a155c3a2e");
   
   if resp.status().is_success() {
      println!("success!");
  } else if resp.status().is_server_error() {
      println!("server error!");
  } else {
      println!("Something else happened. Status: {:?}", resp.status());
  }

}