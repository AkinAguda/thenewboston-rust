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

    #[tokio::test]
    async fn it_gets_account_balance() {
        let bank: Bank = Bank::new(String::from("http://54.183.16.194"), None);

        let response = bank.get_bank_primary_validator().await;

        match response {
            Ok(primary_validator) => {
                let resp = primary_validator
                    .validator
                    .get_account_balance(
                        "10b8a9c088344001bcd2d419286ff37969fb869f2c17593b902f8fe86e690097",
                    )
                    .await;
                match resp {
                    Ok(balance_res) => {
                        println!("Balance is {:?}", balance_res.balance);
                    }
                    Err(error) => println!("Error getting balance: {}", error),
                }
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
            }
        }
    }

    #[tokio::test]
    async fn it_gets_account_balance_lock() {
        let bank: Bank = Bank::new(String::from("http://54.183.16.194"), None);

        let response = bank.get_bank_primary_validator().await;

        match response {
            Ok(primary_validator) => {
                let resp = primary_validator
                    .validator
                    .get_account_balance_lock(
                        "10b8a9c088344001bcd2d419286ff37969fb869f2c17593b902f8fe86e690097",
                    )
                    .await;
                match resp {
                    Ok(balance_lock_res) => {
                        println!("Balance lock is {:?}", balance_lock_res.balance_lock);
                    }
                    Err(error) => println!("Error getting balance Lock: {}", error),
                }
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
            }
        }
    }
}
