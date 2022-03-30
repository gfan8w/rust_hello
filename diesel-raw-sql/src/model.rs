use crate::structs;
use diesel::prelude::*;
use diesel::sql_query;

pub fn getdbconn() -> PgConnection {
    let database_url = "postgres://postgres:postgres@localhost/graphql_todos_example";
    PgConnection::establish(&database_url).unwrap()
}

pub fn list_users() -> Vec<structs::User> {
    let conn = getdbconn();

    let results = sql_query(
        "SELECT
    id,
    firstname,
    lastname,
    email,
    CONCAT(firstname, lastname) as fullname
  FROM
    user_login",
    )
    .load::<structs::User>(&conn)
    .unwrap();
    return results;
}


/* 手动在DB里建表：
CREATE TABLE user_login(
id bigserial PRIMARY KEY ,
firstname varchar(50) NOT NULL,
lastname varchar(50) NOT NULL,
email varchar(100) NOT NULL
);
*/
