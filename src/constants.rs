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
            &Self::Https => "https"
        }
    }
}