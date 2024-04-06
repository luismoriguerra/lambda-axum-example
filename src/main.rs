pub mod http;

use axum::{routing::get, Router};
use lambda_http::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(|| async { "Hello axum" }))
        .merge(http::books::router());

    run(app).await
}
