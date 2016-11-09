extern crate ratel;
extern crate iron;
extern crate json;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use ratel::error::ParseError;
const DEFAULT_PORT: u16 = 3000;

fn compile(source: String, minify: bool, get_ast: bool) -> Result<String, ParseError> {
    let mut ast = match ratel::parser::parse(source) {
        Ok(ast)    => ast,
        Err(error) => return Err(error),
    };

    let settings = ratel::transformer::Settings::target_es5();
    ratel::transformer::transform(&mut ast, settings);

    if get_ast {
        return Ok(format!("{:#?}", ast));
    }

    Ok(ratel::codegen::generate_code(&ast, minify))
}

fn main() {
    fn handler(req: &mut Request) -> IronResult<Response> {

        let mut payload = String::new();

        match req.body.read_to_string(&mut payload) {
            Ok(_)      => {},
            Err(_)     => {
              return Ok(Response::with((status::BadRequest, "Cannot parse request payload")));
            }
        };

        let mut payload = match json::parse(&payload.as_str()) {
            Ok(value)   => value,
            Err(_)      => {
                return Ok(Response::with((status::UnprocessableEntity, "Cannot parse JSON.")));
            }
        };

        let source = match payload["source"].take_string() {
            Some(value)    => value,
            None           => {
                return Ok(Response::with((status::BadRequest, "No source provided")));
            }
        };

        let minify = payload["minify"].as_bool().unwrap_or(false);
        let get_ast = payload["ast"].as_bool().unwrap_or(false);

        let response = match compile(source, minify, get_ast) {
            Ok(result)        => Response::with((status::Ok, result)),
            Err(err)          => Response::with((status::UnprocessableEntity, format!("{:#?}", err)))
        };

        Ok(response)
    }


    let port = option_env!("PORT").map(|port| port.parse().expect("Invalid port"))
                                  .unwrap_or(DEFAULT_PORT);

    let address = format!("0.0.0.0:{}", port);

    let _server = Iron::new(handler).http(address.as_str()).expect("Cannot start the server");

    println!("Listening on port {}", port);
}
