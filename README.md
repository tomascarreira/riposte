# Riposte

A server that allows clients to specify the response code that the server will return.

This could be useful for testing the logic of a client for rare or difficult to set up status code.

## Instalation

**Update when its publish on crates.io**

## Usage

Start the server (the port number can be specified, default is 8080)

    riposte -p 8000

Make a request to the server which includes the header `riposte-request: <code status>`

The code status must be in the range 100-999

The server will respond with that status code and the response will have the header `riposte-response: <the status code specified>`
