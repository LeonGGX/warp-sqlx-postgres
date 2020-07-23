//src/handlers.rs

use sqlx::PgPool;

use warp::{reject, Reply, Rejection};
use warp::http::StatusCode;

use crate::models::{Person, InsertablePerson};
use crate::db;


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

pub async fn add_person_hdler(pers: Person, pool: PgPool) -> Result<impl Reply, Rejection> {

    let insert_pers = InsertablePerson{first_name: pers.first_name, last_name: pers.last_name};

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