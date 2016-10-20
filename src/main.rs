extern crate ratel;
extern crate iron;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use ratel::error::ParseError;

const DEFAULT_PORT: u16 = 3000;

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


    let port = option_env!("PORT").map(|port| port.parse().expect("Invalid port"))
                                  .unwrap_or(DEFAULT_PORT);

    let address = format!("0.0.0.0:{}", port);

    let _server = Iron::new(handler).http(address.as_str()).expect("Cannot start the server");

    println!("Listening on port {}", port);
}
