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
    use constants::{TEST_ACCOUNT_NUMBER, TEST_BANK_IP};
    use server_node::ServerNodeTrait;

    #[tokio::test]
    async fn it_gets_account_balance() {
        let bank: Bank = Bank::new(String::from(TEST_BANK_IP), None);

        let response = bank.get_bank_primary_validator().await;

        match response {
            Ok(primary_validator) => {
                let resp = primary_validator
                    .validator
                    .get_account_balance(TEST_ACCOUNT_NUMBER)
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
        let bank: Bank = Bank::new(String::from(TEST_BANK_IP), None);

        let response = bank.get_bank_primary_validator().await;

        match response {
            Ok(primary_validator) => {
                let resp = primary_validator
                    .validator
                    .get_account_balance_lock(TEST_ACCOUNT_NUMBER)
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
