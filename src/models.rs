use diesel::{EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::schemas::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(i: i32, n: String) -> User {
        User { id: i, name: n }
    }
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
        users::table.load(conn)
    }
    pub fn insert(conn: &MysqlConnection, records: Vec<User>) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(records)
            .execute(conn)
    }
    pub fn get_by_id(conn: &MysqlConnection, i: i32) -> QueryResult<Vec<User>> {
        users::dsl::users
            .filter(users::dsl::id.eq_all(i))
            .load(conn)
    }
}
