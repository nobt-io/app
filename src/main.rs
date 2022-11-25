use std::net::SocketAddr;
use anyhow::{Context, Result};
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT").as_deref().unwrap_or("3000").parse().context("failed to parse port")?;

    let app = Router::new()
        .route("/", get(index));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        title: "Swedish Shenanigans".to_string(),
        currency: "EUR".to_string(),
        total: 1521.0,
        num_participants: 4,
    }
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    currency: String,
    total: f64,
    num_participants: u32
}
