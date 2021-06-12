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

    use tokio::runtime::Runtime;
    #[test]
    fn it_works() {
        let rt = Runtime::new().unwrap();

        let bank: Bank = Bank::new("http://54.177.121.3", None);

        let future = bank.get_bank_primary_validator();
        match rt.block_on(future) {
            resp => {
                println!("{:?}", resp);
            }
            (_) => {
                println!("Errr");
            }
        };
    }
}
