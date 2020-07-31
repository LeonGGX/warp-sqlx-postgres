# warp-sqlx-postgres
a warp server accessing a postgresql database through sqlx

Simple CRUD operations can be made (create, update, delete, and find one person)
the postgresql DB is called "persons".

structs used : 
Person {
  id: i32,
  first_name: String,
  last_name: String
}

