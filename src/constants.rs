use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
pub enum Protocol {
    Http,
    Https,
}
