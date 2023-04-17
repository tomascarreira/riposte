# Riposte

A server that allows clients to specify the response code that the server will return.

This could be useful for testing the logic of a client for rare or difficult to set up status code.

## Instalation

Currently only linux is supported.

Download the latest version from the [releases page](https://github.com/tomascarreira/risposte/releases).

### Or build from source

1. Instal [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (the rust package manager)
2. `cargo install riposte`

The binary will be on .cargo/bin/riposte.

## Usage

Start the server (the port number can be specified, default is 8080)

    riposte -p 8000

Make a request to the server which includes the header `riposte-request: <code status>`

Only http1.1 is currently supported

The request method can be anything and the path can also be anything 

The code status must be in the range 100-999

The server will respond with that status code and the response will have the header `riposte-response: <the status code specified>`

## Examples

Request:

    GET / HTTP/1.1
    Host: 127.0.0.1:8080
    riposte-request: 418

Response:

    HTTP/1.1 418 I'm a teapot
    riposte-renspose: 418
    content-length: 0

Request:

    POST /baz HTTP/1.1
    Host: foo.bar
    Content-Type: application/x-www-form-urlencoded
    Content-Length: 27
    riposte-response: 502

    field1=val1&field2=val2

Response:

    HTTP/1.1 502 Bad Gateway
    riposte-response: 502
    content-lenght: 0

