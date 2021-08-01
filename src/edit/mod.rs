mod edit_schema;

use diesel::{EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use edit_schema::edits;
use edits::dsl as all_edits;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct Edit {
    edit_id: Vec<u8>,
    start: i32,
    end: i32,
    last_edit_id: i32,
    md_content: String,
    book_id: Vec<u8>,
}

impl Edit {
    pub fn new(
        edit_id: Vec<u8>,
        start: i32,
        end: i32,
        last_edit_id: i32,
        md_content: String,
        book_id: Vec<u8>,
    ) -> Self {
        Self {
            edit_id,
            start,
            end,
            last_edit_id,
            md_content,
            book_id,
        }
    }

    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Edit>> {
        all_edits::edits.load(conn)
    }

    pub fn get_by_id(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<Vec<Edit>> {
        all_edits::edits
            .filter(all_edits::edit_id.eq_all(id))
            .load(conn)
    }

    pub fn remove(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<usize> {
        diesel::delete(all_edits::edits)
            .filter(all_edits::edit_id.eq_all(id))
            .execute(conn)
    }
}
