use diesel::{
    sql_types::{Integer, Nullable, Text},
    EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl,
};

use crate::schemas::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, QueryId, QueryableByName)]
pub struct User {
    #[sql_type = "Nullable<Integer>"]
    id: Option<i32>,
    #[sql_type = "Text"]
    name: String,
}

impl User {
    pub fn new(n: String) -> User {
        User { id: None, name: n }
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
