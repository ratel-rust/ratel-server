![Ratel](http://maciej.codes/things/ratel-400.png)

# ratel-server

**Ratel** is a high performance JavaScript compiler with a Rust core.

This is a very simple HTTP server that will accept ES2015+ JavaScript and responds with ES5 compiled code.

## Usage

ratel-server implements a JSON API using [json-rust](https://github.com/maciejhirsz/json-rust).

````json
{
  "source": "const foo=\"\bar"",
  "minify": true,
  "ast": false
}
````

## Logo

The smirky **Ratel** by the courtesy of [A. L. Palmer](https://www.behance.net/alpalmer60b4).

## License

[LICENSE-MIT](LICENSE-MIT)
