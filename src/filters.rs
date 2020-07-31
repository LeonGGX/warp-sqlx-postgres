// src/filters

use crate::handlers;
use crate::models;
use warp::Filter;
use sqlx::PgPool;


fn with_db(pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn post_json() -> impl Filter<Extract = (models::InsertablePerson,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body() -> impl Filter<Extract = (models::InsertablePerson,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


pub fn person_filters(
    pool: PgPool
)-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_persons(pool.clone())
        .or(add_person(pool.clone()))
        .or(find_one_person_id(pool.clone()))
        .or(delete_person(pool.clone()))
        .or(update_person(pool.clone()))

}

pub fn list_persons(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("persons")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::list_persons_hdler)
}


pub fn add_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("persons")
        .and(warp::post())
        .and(post_json())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler)
}

pub fn update_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("persons")
        .and(warp::put())
        .and(warp::path::param())
        .and(json_body())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler)
}

pub fn delete_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("persons")
        .and(warp::delete())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_person_hdler)
}

pub fn find_one_person_id (
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("persons")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::find_person_by_id_hdler)
}



