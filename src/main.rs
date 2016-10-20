extern crate ratel;
extern crate iron;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use ratel::error::ParseError;

fn compile(source: String, minify: bool) -> Result<String, ParseError> {
    let mut ast = match ratel::parser::parse(source) {
        Ok(ast)    => ast,
        Err(error) => return Err(error),
    };

    let settings = ratel::transformer::Settings::target_es5();
    ratel::transformer::transform(&mut ast, settings);

    Ok(ratel::codegen::generate_code(&ast, minify))
}

fn main() {
    fn handler(req: &mut Request) -> IronResult<Response> {
        let mut source = String::new();

        match req.body.read_to_string(&mut source) {
            Ok(_)  => {},
            Err(_) => return Ok(Response::with((status::BadRequest, ":("))),
        }

        Ok(match compile(source, false) {
            Ok(source) => Response::with((status::Ok, source)),
            Err(error) => Response::with((status::UnprocessableEntity, format!("{:?}", error))),
        })
    }

    let _server = Iron::new(handler).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
