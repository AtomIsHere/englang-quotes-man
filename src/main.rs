use std::{net::SocketAddr, env};

use axum::{routing::get, Router};
use dotenvy::dotenv;

use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root};

mod error;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt::init();

    let db = Surreal::new::<Ws>(env::var("DB_CONNECTION")
        .expect("DB_CONNECTION enviroment variable not present"))
        .await?;

    db.signin(Root {
        username: &env::var("DB_USERNAME").expect("DB_USERNAME enviroment variable not present"),
        password: &env::var("DB_PASSWORD").expect("DB_PASSWORD enviroment variable not present"),
    }).await?;

    db.use_ns(env::var("DB_NAMESPACE").expect("DB_NAMESPACE enviroment variable not present"))
        .use_db(env::var("DB_DATABASE").expect("DB_DATABASE enviroment variable not present"))
        .await?;

    let app = Router::new()
        .route("/", get(root))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
