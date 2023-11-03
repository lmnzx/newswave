use anyhow::{Context, Result};
use maud::html;
use serde::Deserialize;
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Deserialize)]
struct FormData {
    email: String,
    name: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_subscription(req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;
    println!("{}", address);
    let mut router = Router::new();
    router.post("/subscribe", api::hello_user);
    router.any("/*", api::wildcard404);
    router.handle(req)
}

mod api {

    use super::*;

    // /subscribe
    pub fn hello_user(req: Request, _params: Params) -> Result<Response> {
        let form_data: String = req
            .body()
            .as_ref()
            .map(|b| String::from_utf8_lossy(&b).into())
            .unwrap();

        let form_data = serde_urlencoded::from_str::<FormData>(&form_data)
            .context("failed to parse form data")?;

        println!("{:?}", form_data);

        let markup = html!(
            p { "Hi," (form_data.name) "!"}
            p { "Your mail is," (form_data.email)}
        )
        .into_string();

        Ok(http::Response::builder()
            .status(200)
            .body(Some(markup.into()))?)
    }

    // wildcard
    pub fn wildcard404(_req: Request, _params: Params) -> Result<Response> {
        let markup = html!(
                h1 { "400 oops!!!"}
        )
        .into_string();

        Ok(http::Response::builder()
            .status(200)
            .body(Some(markup.into()))?)
    }
}
