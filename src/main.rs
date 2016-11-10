extern crate ratel;
extern crate iron;

#[macro_use]
extern crate json;

use std::env;
use std::str::FromStr;
use std::net::Ipv4Addr;
use std::io::Read;
use iron::prelude::*;
use iron::{headers, status};
use iron::method::Method;
use iron::middleware::AfterMiddleware;
use ratel::error::ParseError;

const PROTOCOL: &'static str = "http";
const DEFAULT_HOST: &'static str = "0.0.0.0";
const DEFAULT_PORT: u16 = 3000;

struct CorsMiddleware;

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, _: &mut Request, mut response: Response) -> IronResult<Response> {
        let cors_methods: Vec<Method> = vec![Method::Options, Method::Get, Method::Post];
        response.headers.set(headers::AccessControlAllowOrigin::Any);
        response.headers.set(headers::AccessControlAllowMethods(cors_methods));
        Ok(response)
    }
}

fn get_json_response(error_code: iron::status::Status, payload: String) -> IronResult<Response> {
    let object = object!{
        "success"  => error_code == status::Ok,
        "result"   => payload
    };
    let mut response: Response = Response::with((error_code, object.dump()));
    response.headers.set(headers::ContentType::json());
    Ok(response)
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

    let mut chain = Chain::new(handler);

    if env::var("CORS").is_ok() {
        chain.link_after(CorsMiddleware);
    }

    let host = match env::var("HOST") {
        Ok(value) => Ipv4Addr::from_str(&value).expect("Invalid IPv4 address"),
        _         => Ipv4Addr::from_str(&DEFAULT_HOST).unwrap()
    };

    let port = match env::var("PORT") {
        Ok(value) => value.parse::<u16>().expect("Invalid port"),
        _         => DEFAULT_PORT
    };

    let address = format!("{}:{}", host, port);

    let _server = Iron::new(chain).http(address.as_str()).expect("Cannot start the server");

    println!("Listening on {}://{}:{}", PROTOCOL, host, port);
}
