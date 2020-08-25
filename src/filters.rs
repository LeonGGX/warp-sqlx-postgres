// src/filters

use warp::{Filter, Reply, Rejection};
use sqlx::PgPool;
use warp::filters::BoxedFilter;
use std::error::Error;

//use crate::views;
use crate::handlers;
use crate::models;
use futures::TryFutureExt;
use tera::Context;
use std::collections::HashMap;
use crate::models::InsertablePerson;
use crate::template_setup::tera::render;
use crate::handlers::{page_add_hdler, ServerError};

//******************************************************
// Helper Filters
//

fn with_db(pool: PgPool) -> BoxedFilter<(PgPool,)> {
    warp::any()
        .map(move || pool.clone()).boxed()
}

fn json_body() -> BoxedFilter<(InsertablePerson,)> {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json()).boxed()
}



///
/// function that takes all filters
///
pub async fn person_filters(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    home_page()
        .or(list_persons(pool.clone()))
        .or(add_routes(pool.clone()))
        .or(handle_routes(pool.clone()))
        .boxed()
}

///
/// Filter to display the Home Page
///
pub fn home_page() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::end())
        .and_then(handlers::home_page_hdler)
        .boxed()
}

//*********************************************************
// Filters to add a person to the DB
//*********************************************************

///
/// Filter for the different add routes
/// the first route shows the add page
/// the second route handles the data to add a person to the DB
///
pub fn add_routes(pool: PgPool)-> BoxedFilter<(impl Reply,)> {
    warp::path("add")
        .and(page_add()).or(add_person(pool.clone())).boxed()
}

///
/// Filter to display the add page
/// GET Method
///
pub fn page_add() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::end())
        .and_then(handlers::page_add_hdler)
        .boxed()
}

///
/// Filter to treat adding
/// POST Method
///
pub fn add_person(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::body::form())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler)
        .boxed()
}

//**************************************************************
// filter to handle one person : Show, modify, delete
//**************************************************************

pub fn handle_routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path("persons")
        .and(find_one_person_id(pool.clone())
            .or(update_person(pool.clone()))
            .or(delete_person(pool.clone()))).boxed()
}

pub fn find_one_person_id (pool: PgPool,) -> BoxedFilter<(impl warp::reply::Reply,)> {
        warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::find_person_by_id_hdler).boxed()
}

pub fn update_person(pool: PgPool,) -> BoxedFilter<(impl Reply,)> {
        warp::put()
        .and(warp::path::param())
        .and(json_body())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler).boxed()
}

pub fn delete_person(pool: PgPool,) -> BoxedFilter<(impl warp::reply::Reply,)> {
        warp::delete()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_person_hdler).boxed()
}


pub fn list_persons(pool: PgPool) -> BoxedFilter<(impl warp::reply::Reply,)> {
    warp::path("persons")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::list_persons_hdler).boxed()
}
