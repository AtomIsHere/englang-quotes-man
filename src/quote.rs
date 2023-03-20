use axum::{extract::{State, Path}, Json};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::{model::{Quote, QuoteResponse}, error::Error};

const QUOTE: &str = "quote";

type Db = State<Surreal<Client>>;

pub async fn create(
    db: Db,
    Json(quote): Json<Quote>
) -> Result<Json<Quote>, Error> {
    let quote = db.create(QUOTE).content(quote).await?;
    Ok(Json(quote))
}

pub async fn list(db: Db) -> Result<Json<Vec<QuoteResponse>>, Error> {
    let quotes: Vec<Quote> = db.select(QUOTE).await?;

    Ok(Json(quotes.iter().map(|q| QuoteResponse { id: match q.id.clone() {
        Some(s) => s,
        None => "NOT_PRESENT".to_string(),
    }, quote: q.clone() }).collect()))
}
