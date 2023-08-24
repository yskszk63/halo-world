use std::convert::Infallible;
use std::env;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use tokio::net::{TcpListener, TcpStream};

async fn halo(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("😇")))
}

async fn service(stream: TcpStream) -> anyhow::Result<()> {
    Http::new()
        .http1_only(true)
        .http1_keep_alive(false)
        .serve_connection(stream, service_fn(halo))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let addr = listener.local_addr()?;
    println!("Listening {addr} ...");
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            service(stream)
                .await
                .unwrap_or_else(|err| println!("{err}"))
        });
    }
}
