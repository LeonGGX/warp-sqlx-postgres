//src/main.rs

mod handlers;
mod db;
mod models;
mod errors;

use std::convert::Infallible;
use std::env;

#[macro_use]
extern crate log;

use warp::{Filter};
use sqlx::PgPool;

use env_logger;

use crate::models::{Person, InsertablePerson};


#[tokio::main]
async fn main() {

    env_logger::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();

    let persons = warp::path("persons");

    let list_persons = persons
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::list_persons_hdler);

    let add_person =  persons
        .and(warp::post())
        //.and(warp::body::json())
        .and(post_json())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler);

    let find_one_person_id = persons
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::find_person_by_id_hdler);

    let persons_routes = list_persons.or(add_person).or(find_one_person_id);

    warp::serve(persons_routes)
        .run(([127, 0, 0, 1], 8085))
        .await
}

fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    //let clone = db_pool.clone();
    warp::any().map( move || db_pool.clone())
}

fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json() -> impl Filter<Extract = (i32,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}



