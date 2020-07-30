//src/main.rs


use std::convert::Infallible;
use std::env;

#[macro_use]
extern crate log;

use warp::{Filter};
use sqlx::PgPool;

use env_logger;
use tracing_subscriber::fmt::format::FmtSpan;

use futures::TryFutureExt;
use tracing::Level;

use crate::db::update_person;
use crate::models::{Person, InsertablePerson};

mod handlers;
mod db;
mod models;
mod errors;
//mod routes;
//mod api;


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
    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");

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

    let delete_person = persons
        .and(warp::delete())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_person_hdler);

    let update_person = persons
        .and(warp::put())
        .and(warp::body::json())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler);


    let persons_api = list_persons
        .or(add_person)
        .or(find_one_person_id)
        .or(delete_person)
        .or(update_person)
        .recover(handlers::handle_rejection);


    warp::serve(persons_api)
        .run(([127, 0, 0, 1], 8085))
        .await
}


fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    //let clone = db_pool.clone();
    warp::any().map( move || db_pool.clone())
}

/*
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
*/
fn post_json() -> impl Filter<Extract = (InsertablePerson,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/*
#[tokio::test]
async fn query_struct() {
    let as_struct = warp::query::<Person>();

    let req = warp::test::request().path("/?id=7&first_name=Barry&last_name=CONNELY");

    let extracted = req.filter(&as_struct).await.unwrap();
    assert_eq!(
        extracted,
        Person {
            id: 7,
            first_name: "Barry".into(),
            last_name: "CONNELY".into()
        }
    );
}
*/
#[tokio::test]
async fn modify_person() {

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();

    let ins_pers =
        InsertablePerson{first_name: "James".to_string(), last_name: "ANDERSONIAN".to_string()};

    let persons = warp::path("persons");
    let update_person = persons
        .and(warp::put())
        .and(warp::body::json())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler);


    let req = warp::test::request()
        .method("PUT")
        .path("http://127.0.0.1:8085/persons/8")
        .json(&ins_pers)
        .header("accept", "application/json")
        .reply(&update_person)
        .await;

    assert_eq!(req.status(), 202, "Should return 202 OK.");
    println!("{:#?}", req.body());

}

#[tokio::test]
async fn post_person() {

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();

    let ins_pers =
        InsertablePerson{first_name: "Bernie".to_string(), last_name: "ECKELSTONE".to_string()};

    let persons = warp::path("persons");
    let add_person =  persons
        .and(warp::post())
        .and(post_json())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler);

    let req = warp::test::request()
        .method("POST")
        .path("http://127.0.0.1:8085/persons")
        .json(&ins_pers)
        .header("accept", "application/json")
        .reply(&add_person)
        .await;

    assert_eq!(req.status(), 201, "Should return 201 CREATED.");
    tracing::info!("{:#?}", req.body());
}

#[tokio::test]
async fn delete_person() {

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");

    let db_url = "postgres://postgres:1922leon@localhost/persons";
    let pool = db::create_pg_pool(db_url).await.unwrap();

    let persons = warp::path("persons");
    let delete_person = persons
        .and(warp::delete())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_person_hdler);

    let req = warp::test::request()
        .method("DELETE")
        .path("http://127.0.0.1:8085/persons/12")
        .header("accept", "application/json")
        .reply(&delete_person)
        .await;

    assert_eq!(req.status(), 202, "Should return 202 ACCEPTED.");
    tracing::info!("{:#?}", req.body());

}



