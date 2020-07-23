// src/models.rs
use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

use warp::reply::Response;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct InsertablePerson {
    pub first_name : String,
    pub last_name : String,
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
}

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

// si on veut une sortie String et non Json ...
// donc pas très utile.
impl warp::reply::Reply for Person {
    fn into_response(self) -> Response {
        Response::new(format!("id: {}\n nom: {}\n prenom: {}", self.id, self.first_name, self.last_name).into())
    }
}
