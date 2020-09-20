// src/filters

use warp::{Filter, Reply,};
use sqlx::PgPool;
use warp::filters::BoxedFilter;

use crate::handlers;
use crate::models::InsertablePerson;


///
/// Main Filter
/// function that takes all filters
///
pub async fn person_filters(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    handle_routes(pool.clone())
        .or(add_routes(pool.clone()))
        .or(page_list(pool.clone()))
        .or(page_home()).boxed()
}

///
/// Filter for the different add routes
/// the first route shows the add page
/// the second route handles the data to add a person to the DB
///
fn add_routes(pool: PgPool)-> BoxedFilter<(impl Reply,)> {
    page_add()
        .or(add_person(pool.clone()))
        .boxed()
}

fn handle_routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    page_modify(pool.clone())
        .or(update_person(pool.clone()))
        .or(delete_person(pool.clone()))
        .boxed()
}


///
/// Filter to display the Home Page
///
fn page_home() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::end())
        .and_then(handlers::page_home_hdler)
        .boxed()
}
///
/// Filter to display the list page
/// GET Method
///
fn page_list(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("persons"))
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::list_persons_hdler)
        .boxed()
}
///
/// Filter to display the add page
/// GET Method
///
fn page_add() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("add"))
        .and(warp::path::end())
        .and_then(handlers::page_add_hdler)
        .boxed()
}
///
/// Filter to display the modify page
/// GET Method
///
fn page_modify(pool: PgPool) -> BoxedFilter<(impl Reply,)>{
    warp::get()
        .and(warp::path("persons"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::find_person_by_id_hdler)
        .boxed()
}

///
/// Filter to treat adding
/// POST Method
///
fn add_person(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path("add"))
        .and(warp::body::form())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler)
        .boxed()
}


fn update_person(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::put()
        .and(warp::path("persons"))
        .and(warp::path::param::<i32>())
        .and(warp::body::form())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler)
        .boxed()
}

fn delete_person(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::delete()
        .and(warp::path("persons"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone())).boxed()
        .and_then(handlers::delete_person_hdler)
        .boxed()
}


//******************************************************
// Helper Filters
//******************************************************

fn with_db(pool: PgPool) -> BoxedFilter<(PgPool,)> {
    warp::any()
        .map(move || pool.clone())
        .boxed()
}

fn json_body() -> BoxedFilter<(InsertablePerson,)> {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
        .boxed()
}

