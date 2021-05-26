use reqwest::{ get, Response, Error };

/** The pagination options that can be sent with requests and the instantiation of a new server node class. */
pub struct PaginationOptions {
    pub limit: u32,
    pub offset: u32,
  }
  
/** The options that are available for all server nodes. */
pub struct ServerNodeOptions {
    pub defaultPagination: PaginationOptions
}

pub struct ServerNode<'a> {
    pub url: &'a str,
    options: Option<ServerNodeOptions>
}

impl<'a> ServerNode<'a> {
    pub async fn get_data(&self, endpoint: &str) -> Result<Response, Error> {
        let body = get(format!("{}{}", self.url, endpoint)).await;
        body
    }
}

trait ServerNodeTrait {
    fn greet() {
        print!("hi")
    }
}