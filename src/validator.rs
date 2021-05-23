mod server_node;
use  server_node::{ ServerNode };

pub struct Validator {
    a: i32,
    ..ServerNode,
}