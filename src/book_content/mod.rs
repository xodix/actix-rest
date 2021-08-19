mod book_content_schema;
use bc::dsl as all_bc;
use book_content as bc;
use book_content_schema::book_content;
use diesel::{MysqlConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

/// Book content corresponding to the book_content table
#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "book_content"]
pub struct BookContent {
    /// hex 16 byte uuid
    content_id: Vec<u8>,
    /// content in markdown format
    md_content: String,
    /// last edit hex 16 byte uuid
    last_edit: Vec<u8>,
}

impl BookContent {
    /// Creates new BookContent
    pub fn new(content_id: Vec<u8>, md_content: String, last_edit: Vec<u8>) -> Self {
        Self {
            content_id,
            md_content,
            last_edit,
        }
    }

    /// gets all content
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<BookContent>> {
        all_bc::book_content.load(conn)
    }

    /// inserts book content into the book_content table
    pub fn insert(conn: &MysqlConnection, vals: Vec<BookContent>) -> QueryResult<usize> {
        diesel::insert_into(all_bc::book_content)
            .values(vals)
            .execute(conn)
    }
}
