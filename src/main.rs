use std::{net::SocketAddr, str::FromStr};

use http_body_util::Full;
use hyper::{server, service, Request, body, Response, StatusCode};

async fn handle_request(req: Request<body::Incoming>) -> anyhow::Result<Response<Full<body::Bytes>>> {
    println!("{:?}", req);
    let status_code = match req.headers().get("riposte-request") {
        Some(val) => val,
        None => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Full::new(body::Bytes::new()))?),
    };

    let status_code = match status_code.to_str() {
        Ok(val) => val,
        Err(_) => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Full::new(body::Bytes::new()))?),
    };

    let status_code = match StatusCode::try_from(status_code) {
        Ok(val) => val,
        Err(_) => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Full::new(body::Bytes::new()))?),
    };

    Ok(Response::builder().status(status_code).header("riposte-response", &status_code.to_string()).body(Full::new(body::Bytes::new()))?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from_str("127.0.0.1:8080")?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on address: {addr}");

    loop {
        let (stream, _) = listener.accept().await?;
        println!("Connection accepted.");

        tokio::task::spawn(async move {
            if let Err(err) = server::conn::http1::Builder::new()
                .serve_connection(stream, service::service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
