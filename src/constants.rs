use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Bank,
    ConfirmationValidator,
    PrimaryValidator,
}

pub type Hex = String;

pub type Origin = String;

pub type Url = String;

pub type Trust = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

impl Protocol {
    pub fn get_str(&self) -> &str {
        match self {
            &Self::Http => "http",
            &Self::Https => "https",
        }
    }
}
pub const TEST_ACCOUNT_NUMBER: &str =
    "10b8a9c088344001bcd2d419286ff37969fb869f2c17593b902f8fe86e690097";

pub const TEST_BANK_IP: &str = "http://54.183.16.194";
