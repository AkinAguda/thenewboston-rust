pub enum NodeType {
    Bank,
    ConfirmationValidator,
    PrimaryValidator,
}

pub type Hex<'a> = &'a str;

pub type Origin<'a> = &'a str;

pub type Url<'a> = &'a str;

pub type Trust<'a> = &'a str;

pub enum Protocol {
    Http,
    Https,
}
