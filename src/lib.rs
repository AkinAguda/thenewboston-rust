mod bank;
mod models;
mod primary_validator;
mod server_node;
mod validator;

#[cfg(test)]
mod tests {
    use super::*;

    use bank::Bank;
    use server_node::ServerNodeTrait;

    use tokio::runtime::Runtime;
    #[test]
    fn it_works() {
        let rt = Runtime::new().unwrap();

        let bank: Bank = Bank::new("http://54.177.121.3", None);

        let future = bank.get_config();
        match rt.block_on(future) {
            Ok(resp) => {
                let text = resp.text();
                match rt.block_on(text) {
                    Ok(value) => println!("{}", value),
                    Err(error) => println!("{}", error),
                }
            }
            Err(_) => {
                println!("Errr");
            }
        };
    }
}
