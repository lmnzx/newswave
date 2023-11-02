use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_subscription(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .body(Some("Hello from the subsription component".into()))?)
}
