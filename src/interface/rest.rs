use axum::{routing::get, Router};

pub async fn start() {
    let app = Router::new().route("/", get(root));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
