//src/handlers.rs

use std::convert::Infallible;
use std::error::Error;

use serde::Serialize;

use sqlx::PgPool;

use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

use tera::{Context};

use crate::db;
use crate::errors::CustError;
use crate::models::{InsertablePerson,};

use crate::template_setup::tera::render;
use warp::reject::Reject;
use std::fmt::Display;

use thiserror::Error;

pub async fn page_home_hdler() -> Result<Box<dyn Reply>, Rejection> {
    tracing::info!("HDLR : chargement page home");
    let ctx = Context::new();
    let body = render("index.html", &ctx).unwrap();
    Ok(Box::new(warp::reply::html(body)))
}

pub async fn page_add_hdler() -> Result<Box<dyn Reply>, Rejection> {
    tracing::info!("HDLR : chargement page add");
    let ctx = Context::new();
    let body = render("add_person.html", &ctx).unwrap();
    Ok(Box::new(warp::reply::html(body)))
}

pub async fn find_person_by_id_hdler(id: i32, pool: PgPool,) -> Result<Box<dyn Reply>, Rejection> {
    let res = db::find_person_by_id(id, &pool).await;
    match res {
        Ok(person) => {
            tracing::info!("HDLR : Personne trouvée : {}, {}", &person.last_name, &person.first_name);

            let mut ctx = Context::new();
            ctx.insert("person", &person);

            let body = render("modify_person.html", &ctx).unwrap();
            tracing::info!("chargement page modify");
            Ok(Box::new(warp::reply::html(body)))
        },
        Err(_) => {
            tracing::info!("HDLR : Erreur: personne pas trouvée !");
            Err(reject::not_found())
        },
    }
}

///
/// Handles the request to show a list of persons in the DB
/// Shows the list in the Tera template
///
pub async fn list_persons_hdler(pool: PgPool,) -> Result<Box<dyn Reply>, Rejection> {
    let res = db::list_persons(&pool).await;
    match res {
        Ok(list_persons) => {
            tracing::info!("HDLR : Liste des personnes trouvée");

            let mut ctx = Context::new();
            ctx.insert("persons", &list_persons);
            let body = render("persons.html", &ctx).unwrap();

            Ok(Box::new(warp::reply::html(body)))
        },
        Err(_) => {
            tracing::info!("HDLR : Erreur: liste personne pas trouvée !");
            Err(reject::not_found())
        },
    }
}



///
/// Handles request to add a person to the DB
/// redirects to the list persons page
///
pub async fn add_person_hdler(insert_pers: InsertablePerson, pool: PgPool,) -> Result<Box<dyn Reply>, Rejection> {
    let res = db::add_person(&pool, insert_pers).await;
    match res {
        Ok(pers) => {
            tracing::info!("HDLR : created person : {:?}", &pers);
            Ok(list_persons_hdler(pool.clone()).await.unwrap())
        }
        Err(err) => {
            let error = ErrorMessage { code: 405, message: "HDLR : erreur création personne".to_string() };
            Ok(Box::new(warp::reply::json(&error)))
        }
    }
}

pub async fn delete_person_hdler(pers_id: i32, pool: PgPool) -> Result<Box<dyn Reply>, Rejection> {
    let res = db::delete_person(pers_id, &pool).await;
    match res {
        Ok(id) => {
            tracing::info!("HDLR : id person deleted : {:?}", &id);
            Ok(list_persons_hdler(pool.clone()).await.unwrap())
        }
        Err(_) => {
            tracing::info!("HDLR : error deleting person");
            Ok(Box::new(StatusCode::BAD_REQUEST))
        }
    }
}

pub async fn update_person_hdler(
    pers_id: i32,
    modifyed_pers: InsertablePerson,
    pool: PgPool,
) -> Result<Box<dyn Reply>, Rejection> {

    tracing::info!("HDLR : Person send to handler update: {:?}", &modifyed_pers);

    let res = db::update_person(pers_id, modifyed_pers, &pool).await;
    match res {
        Ok(pers) => {
            tracing::info!(" HDLR : Person updated : {:?}", &pers);
            Ok(list_persons_hdler(pool.clone()).await.unwrap())
        }
        Err(_) => {
            tracing::info!("HDLR : error updating person");
            Ok(Box::new(StatusCode::BAD_REQUEST))
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
pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
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
            CustError::DBQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request DBQueryError";
            }
            CustError::DBPoolError(_) => {
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

    Ok(Box::new(warp::reply::with_status(json, code)))
}

async fn customize_error(err: Rejection) -> Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;

    if let Some(server_error) = err.find::<ServerError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = server_error.msg.to_owned();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }
    Ok(Box::new(warp::reply::with_status(message, code)))
}

#[derive(Debug, Clone, Error, Serialize, PartialEq)]
pub struct ServerError {
    #[serde(skip)]
    pub status: StatusCode,
    pub msg: String,
}

impl Reject for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.msg, self.status)
    }
}

impl ServerError {
    pub fn new(status: StatusCode, msg: &str) -> Self {
        ServerError {
            status,
            msg: msg.to_owned(),
        }
    }
}

impl From<anyhow::Error> for ServerError {
    fn from(err: anyhow::Error) -> Self {
        let e = match err.downcast::<ServerError>() {
            Ok(e) => return e,
            Err(e) => e,
        };

        ServerError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Unhandled error type: {:#?}", e),
        )
    }
}
