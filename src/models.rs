// src/models.rs

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use std::str::FromStr;

use sqlx::postgres::PgRow;
use warp::reply::Response;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct InsertablePerson {
    pub first_name: String,
    pub last_name: String,
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.last_name);
        str.push_str(" ");
        str.push_str(&self.first_name);
        str
    }
    /*
       pub fn add_person(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
           let mut tx = pool.acquire();
           let rec = sqlx::query("INSERT INTO persons (first_name, last_name)
                   VALUES ( $1, $2 )
                   RETURNING id, first_name, last_name;"
           )
               .bind(&pers.first_name)
               .bind(&pers.last_name)
               .map(|row:PgRow| {
                   Person {
                       id: row.get(0),
                       first_name: row.get(1),
                       last_name: row.get(2)
                   }
               })
               .fetch_one(&mut tx)
               .await?;
           tx.commit().await?;

           log::debug!("person added : {:?}", &rec);
           Ok(rec)
       }

    */
}

impl FromStr for InsertablePerson {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InsertablePerson {
            first_name: "".to_string(),
            last_name: "".to_string(),
        })
    }
}

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow, Debug, Eq, PartialEq)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

// si on veut une sortie String et non Json ...
// donc pas très utile.
impl warp::reply::Reply for Person {
    fn into_response(self) -> Response {
        Response::new(
            format!(
                "id: {}\n nom: {}\n prenom: {}",
                self.id, self.first_name, self.last_name
            )
            .into(),
        )
    }
}

impl warp::reply::Reply for InsertablePerson {
    fn into_response(self) -> Response {
        Response::new(format!("nom: {}\n prenom: {}", self.first_name, self.last_name).into())
    }
}
