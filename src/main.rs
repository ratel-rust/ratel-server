extern crate ratel;
extern crate iron;

use std::io::Read;
use iron::prelude::*;
use iron::status;

fn compile(source: String) -> String {
    let mut ast = match ratel::parser::parse(source) {
        Ok(ast)    => ast,
        Err(error) => return format!("{}", error),
    };

    let settings = ratel::transformer::Settings::target_es5();
    ratel::transformer::transform(&mut ast, settings);

    ratel::codegen::generate_code(&ast, false)
}

fn main() {
    fn handler(req: &mut Request) -> IronResult<Response> {
        let mut source = String::new();

        match req.body.read_to_string(&mut source) {
            Ok(_)  => {},
            Err(_) => return Ok(Response::with((status::BadRequest, ":(")))
        }

        Ok(Response::with((status::Ok, compile(source))))
    }

    let _server = Iron::new(handler).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
