use axum::{Router, routing::get};
use axum::response::Html;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::apps::system;

pub fn init()->Router { 
    system::route::init()
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(Level::INFO)),
    )
}



pub async fn root() -> Html<&'static str>{
    Html("<h1>welcome to axum!</h1>")
}