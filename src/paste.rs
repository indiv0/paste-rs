extern crate http;
extern crate iron;

use std::io::net::ip::Ipv4Addr;
use iron::status;
use iron::{
    Iron,
    Request,
    Response,
    IronResult,
};

fn main() {
    Iron::new(hello_world).listen(Ipv4Addr(127, 0, 0, 1), 3000);
    println!("On 3000");
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok, "Hello, world!"))
}
