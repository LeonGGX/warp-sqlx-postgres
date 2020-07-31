// src/db.rs

use sqlx::postgres::PgRow;
use sqlx::{PgConnection, PgPool, Row};

use crate::models::{InsertablePerson, Person};
//use crate::errors;

/// Open a connection to a database
pub async fn create_pg_pool(db_url: &str) -> sqlx::Result<PgPool> {
    let pool = PgPool::new(db_url).await?;
    Ok(pool)
}

/*
pub async fn get_db_con(db_pool: &PgPool) -> anyhow::Result<PgConnection> {
    db_pool.get().await.map_err(errors::CustError::DBPoolError)
}


fn row_to_person(row: &PgRow) -> Person {
    let id: i32 = row.get(0);
    let first_name: String = row.get(1);
    let last_name: String = row.get(2);
    Person {
        id,
        first_name,
        last_name,
    }
}
*/

pub async fn list_persons(pool: &PgPool) -> anyhow::Result<Vec<Person>> {
    let mut tx = pool.begin().await?;

    let mut persons: Vec<Person> = Vec::new();
    let recs: Vec<Person> = sqlx::query(
        "SELECT id, first_name, last_name
                                        FROM persons
                                        ORDER BY id;",
    )
    .map(|row: PgRow| Person {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
    })
    .fetch_all(&mut tx)
    .await?;

    tx.commit().await?;

    for rec in recs {
        persons.push(Person {
            id: rec.id,
            first_name: rec.first_name,
            last_name: rec.last_name,
        });
    }

    Ok(persons)
}

pub async fn find_person_by_id(id: i32, pool: &PgPool) -> anyhow::Result<Person> {
    let mut tx = pool.begin().await?;
    let rec = sqlx::query("SELECT * FROM persons WHERE id = $1;")
        .bind(id)
        .map(|row: PgRow| Person {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
        })
        .fetch_one(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(Person {
        id: rec.id,
        first_name: rec.first_name,
        last_name: rec.last_name,
    })
}

pub async fn add_person(pool: &PgPool, pers: InsertablePerson) -> anyhow::Result<Person> {
    let mut tx = pool.begin().await?;
    let rec = sqlx::query(
        "INSERT INTO persons (first_name, last_name)
                VALUES ( $1, $2 )
                RETURNING id, first_name, last_name;",
    )
    .bind(&pers.first_name)
    .bind(&pers.last_name)
    .map(|row: PgRow| Person {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
    })
    .fetch_one(&mut tx)
    .await?;
    tx.commit().await?;

    log::debug!("person added : {:?}", &rec);
    Ok(rec)
}

pub async fn update_person(
    id: i32,
    update_person: InsertablePerson,
    pool: &PgPool,
) -> anyhow::Result<Person> {
    let mut tx = pool.begin().await.unwrap();
    let person = sqlx::query(
        "UPDATE persons \
                                        SET first_name = $1, \
                                        last_name = $2 \
                                        WHERE id = $3 \
                                        RETURNING id, first_name, last_name;",
    )
    .bind(&update_person.first_name)
    .bind(&update_person.last_name)
    .bind(id)
    .map(|row: PgRow| Person {
        id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
    })
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(person)
}

pub async fn delete_person(id: i32, pool: &PgPool) -> anyhow::Result<i32> {
    let mut tx = pool.begin().await?;
    let res = sqlx::query("DELETE FROM persons WHERE id = $1")
        .bind(id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;
    let deleted = res as i32;
    Ok(deleted)
}
