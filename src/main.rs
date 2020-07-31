//src/main.rs

use std::convert::Infallible;
use std::env;

use sqlx::PgPool;
use warp::Filter;

use tracing_subscriber::fmt::format::FmtSpan;
use tracing::Level;

use crate::db::update_person;
use crate::models::{InsertablePerson, Person};

mod db;
mod errors;
mod handlers;
mod models;
mod filters;

#[tokio::main]
async fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder
        .finish();
    // and sets the constructed `Subscriber` as the default.
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();

    let api = filters::person_filters(pool);
    warp::serve(api).run(([127, 0, 0, 1], 8085)).await
}

#[tokio::test]
async fn modify_person() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();
    let api = filters::person_filters(pool);

    let ins_pers = InsertablePerson {
        first_name: "James".to_string(),
        last_name: "ANDERSON".to_string(),
    };

    let req = warp::test::request()
        .method("PUT")
        .path("http://127.0.0.1:8085/persons/8")
        .json(&ins_pers)
        .header("accept", "application/json")
        .reply(&api)
        .await;

    println!("{:#?}", req.body());
    assert_eq!(req.status(), 202, "Should return 202 OK.");
}

#[tokio::test]
async fn post_person() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();
    let api = filters::person_filters(pool);

    let ins_pers = InsertablePerson {
        first_name: "Enrico".to_string(),
        last_name: "MACIAS".to_string(),
    };

    let req = warp::test::request()
        .method("POST")
        .path("http://127.0.0.1:8085/persons")
        .json(&ins_pers)
        .header("accept", "application/json")
        .reply(&api)
        .await;

    tracing::info!("{:#?}", req.body());
    assert_eq!(req.status(), 201, "Should return 201 CREATED.");
}

#[tokio::test]
async fn delete_person() {

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();
    let api = filters::person_filters(pool);

    let req = warp::test::request()
        .method("DELETE")
        .path("http://127.0.0.1:8085/persons/16")
        .header("accept", "application/json")
        .reply(&api)
        .await;

    tracing::info!("{:#?}", req.body());
    assert_eq!(req.status(), 202, "Should return 202 ACCEPTED.");
}
