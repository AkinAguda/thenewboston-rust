mod bank;
mod constants;
mod models;
mod primary_validator;
mod responses;
mod server_node;
mod validator;

#[cfg(test)]
mod tests {
    use super::*;

    use bank::Bank;
    use server_node::ServerNodeTrait;

    // use tokio::runtime::Runtime;
    #[tokio::test]
    async fn it_works() {
        // let rt = Runtime::new().unwrap();

        let bank: Bank = Bank::new(String::from("http://54.177.121.3"), None);

        let response = bank.get_bank_primary_validator().await;

        match response {
            Ok(primary_validator) => {
                println!("SUCCESS: {:?}", primary_validator.validator.server_node.url);
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
            }
        }
    }
}
