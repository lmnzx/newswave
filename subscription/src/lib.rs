use anyhow::Result;
use maud::html;
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_subscription(req: Request) -> Result<Response> {
    let mut router = Router::new();
    router.get("/subscribe/:user", api::hello_user);
    router.handle(req)
}

mod api {
    use super::*;

    // /subscribe/:user
    pub fn hello_user(_req: Request, params: Params) -> Result<Response> {
        let user = params.get("user").expect("lemon");

        let markup = html!(
            p { "Hi," (user) "!"}
        )
        .into_string();

        Ok(http::Response::builder()
            .status(200)
            .body(Some(markup.into()))?)
    }
}
