![Ratel](http://maciej.codes/things/ratel-400.png)

# ratel-server

**Ratel** is a high performance JavaScript to JavaScript compiler with a Rust core. It's goal is to take newest versions of JavaScript as input, and produce output that's compatible with older versions of the language.

This is a very simple HTTP server that will accept ES2015+ JavaScript and responds with ES5 compiled code.

## Building

Have Rust installed, then execute ``cargo build --release`` in the root
directory of this repository.

## Usage and environment variables

````bash
  $ ./target/release/ratel-server
````

The following environment variables can be used:

| name | default value | description                           |
|------|---------------|---------------------------------------|
| HOST | 0.0.0.0       | binding IPv4 address                  |
| PORT | 3000          | port number                           |
| CORS | false         | Whether to include a CORS header      |

## API

ratel-server implements a JSON API using [json-rust](https://github.com/maciejhirsz/json-rust).

The server responds to any request with a JSON body, f.e.:

````bash
 $ curl 'http://0.0.0.0:3000' \
   -H 'content-type: application/json' \
   -d '{"source":"const foo = 2;\nconst bar = foo**2;","minify":false,"ast":false}'
````

results in:

````json
{
  "result": "var foo = 2;\nvar bar = Math.pow(foo, 2);\n",
  "success": true
}
````

The following options are available.

| key    | mandatory | type      | description                           |
|--------|-----------|-----------|---------------------------------------|
| source | yes       | String    | The JavaScript source code to compile |
| minify | no        | Boolean   | Whether to minify the output          |
| ast    | no        | Boolean   | Whether to return the AST             |

## Logo

The smirky **Ratel** by the courtesy of [A. L. Palmer](https://www.behance.net/alpalmer60b4).

## License

[LICENSE-MIT](LICENSE-MIT)
