//src/handlers.rs

use std::convert::Infallible;
use std::error::Error;

use serde::Serialize;

use sqlx::PgPool;

use warp::{reject, Reply, Rejection};
use warp::http::StatusCode;

use crate::models::{Person, InsertablePerson};
use crate::db;
use crate::errors::CustError;


pub(crate) async fn find_person_by_id_hdler(id: i32, pool: PgPool, ) -> Result<impl Reply, Rejection> {
    let res = db::find_person_by_id(id, &pool).await;
    match res {
        Ok(person) => Ok(warp::reply::json(&person)),
        Err(_) => Err(reject::not_found()),
    }
}

pub async fn list_persons_hdler(pool: PgPool) -> Result<impl Reply, Rejection> {
    let res = db::list_persons(&pool).await;
    match res {
        Ok(list_persons) => Ok(warp::reply::json(&list_persons)),
        Err(_) => Err(reject::not_found())
    }
}

pub async fn add_person_hdler(insert_pers: InsertablePerson, pool: PgPool) -> Result<impl Reply, Rejection> {

    //let insert_pers = InsertablePerson{first_name: pers.first_name, last_name: pers.last_name};

    let res = db::add_person(&pool, insert_pers).await;
    match res {
        Ok(pers) => {
            log::debug!("create person : {:?}", &pers);
            Ok(StatusCode::CREATED)
        },
        Err(_) => {
            log::debug!("error creating person");
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn delete_person_hdler(pers_id: i32, pool: PgPool) -> Result<impl Reply, Rejection> {
    let res = db::delete_person(pers_id, &pool).await;
    match res {
        Ok(id) => {
            log::debug!(" id person deleted : {:?}", &id);
            Ok(StatusCode::ACCEPTED)
        },
        Err(_) => {
            log::debug!("error creating person");
            Ok(StatusCode::BAD_REQUEST)
        }
    }

}

pub async fn update_person_hdler(pers_id: i32, modifyed_pers: InsertablePerson, pool:PgPool)-> Result<impl Reply, Rejection> {
    let res = db::update_person(pers_id, modifyed_pers, &pool).await;
    match res {
        Ok(pers) => {
            debug!(" id person updated : {:?}", &pers);
            Ok(StatusCode::ACCEPTED)
        },
        Err(_) => {
            debug!("error updating person");
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    }
    else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else if let Some(e) = err.find::<CustError>() {
        match e {
            CustError::DBQueryError(_)=> {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request DBQueryError";
            }
            CustError::DBPoolError(_)=> {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request DBPoolError";
            }
            CustError::PgSqlxError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request PgSqlxError";
            }
        }
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
