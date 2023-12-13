use axum::{Router, routing::get, response::Html};

pub fn init() -> Router{
    Router::new()
    .route("/api/", get(api))
    
}

pub async fn api() -> Html<&'static str>{
    Html("<h1>api!</h1>")
}