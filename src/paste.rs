extern crate iron;
extern crate time;
extern crate typemap;

use std::io::net::ip::Ipv4Addr;
use iron::status;
use iron::{
    AfterMiddleware,
    BeforeMiddleware,
    Chain,
    ChainBuilder,
    Iron,
    Request,
    Response,
    IronResult,
};
use time::precise_time_ns;
use typemap::Assoc;

fn main() {
    let mut chain = ChainBuilder::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).listen(Ipv4Addr(127, 0, 0, 1), 3000);
    println!("On 3000");
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok, "Hello, world!"))
}

struct ResponseTime;

impl Assoc<u64> for ResponseTime {}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime, u64>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, _: &mut Response) -> IronResult<()> {
        let delta = precise_time_ns() - *req.extensions.find::<ResponseTime, u64>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(())
    }
}
