extern crate ratel;
extern crate iron;
#[macro_use]
extern crate json;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use ratel::error::ParseError;
const DEFAULT_PORT: u16 = 3000;

fn get_json_response(error_code: iron::status::Status, payload: String) -> IronResult<Response> {
    let object = object!{
        "success"  => error_code == status::Ok,
        "result"   => payload
    };

    Ok(Response::with((error_code, object.dump())))
}

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
            Ok(_)             => {},
            Err(_)            => return get_json_response(status::BadRequest, "Cannot parse request payload".into())
        };

        let mut payload = match json::parse(&payload.as_str()) {
            Ok(value)         => value,
            Err(_)            => return get_json_response(status::BadRequest, "Cannot parse JSON".into())
        };

        let source = match payload["source"].take_string() {
            Some(value)       => value,
            None              => return get_json_response(status::BadRequest, "No source provided".into())
        };

        let minify = payload["minify"].as_bool().unwrap_or(false);
        let get_ast = payload["ast"].as_bool().unwrap_or(false);

        let response = match compile(source, minify, get_ast) {
            Ok(result)        => get_json_response(status::Ok, result),
            Err(err)          => get_json_response(status::UnprocessableEntity, format!("{:#?}", err))
        };

        response
    }


    let port = option_env!("PORT").map(|port| port.parse().expect("Invalid port"))
                                  .unwrap_or(DEFAULT_PORT);

    let address = format!("0.0.0.0:{}", port);

    let _server = Iron::new(handler).http(address.as_str()).expect("Cannot start the server");

    println!("Listening on port {}", port);
}
