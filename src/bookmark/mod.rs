mod bookmark_schema;

use bookmark_schema::bookmarks;
use bookmarks::dsl as all_bookmarks;

use diesel::{EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

/// Bookmark struct corresponding to the bookmark schema
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct Bookmark {
    /// hex 16 byte uuid
    bookmark_id: Vec<u8>,
    /// hex 16 byte bookmarked book uuid
    book_id: Vec<u8>,
    /// hex 16 byte user uuid
    user_id: Vec<u8>,
    /// the character user ended their reading on
    ended_on: i32,
}

impl Bookmark {
    /// Create a new bookmark
    pub fn new(bookmark_id: Vec<u8>, book_id: Vec<u8>, user_id: Vec<u8>, ended_on: i32) -> Self {
        Self {
            bookmark_id,
            book_id,
            user_id,
            ended_on,
        }
    }

    /// get all bookmarks and return Result
    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Bookmark>> {
        all_bookmarks::bookmarks.load(conn)
    }

    /// get the bookmark by id and return Result
    pub fn get_by_id(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<Vec<Bookmark>> {
        all_bookmarks::bookmarks
            .filter(all_bookmarks::bookmark_id.eq_all(id))
            .load(conn)
    }

    /// inser bookmark into bookmarks table and return Result
    pub fn insert(conn: &MysqlConnection, vals: Vec<Bookmark>) -> QueryResult<usize> {
        diesel::insert_into(all_bookmarks::bookmarks)
            .values(vals)
            .execute(conn)
    }

    /// get users proggres and return Result
    pub fn get_progress(
        conn: &MysqlConnection,
        id: Vec<u8>,
        book_id: Vec<u8>,
    ) -> QueryResult<Vec<Bookmark>> {
        all_bookmarks::bookmarks
            .filter(all_bookmarks::user_id.eq_all(id))
            .filter(all_bookmarks::book_id.eq_all(book_id))
            .load(conn)
    }

    /// remove bookmark by id and return Result
    pub fn remove_bookmark(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<usize> {
        diesel::delete(all_bookmarks::bookmarks)
            .filter(all_bookmarks::bookmark_id.eq_all(id))
            .execute(conn)
    }

    /// update users progress and return Result
    pub fn update_progress(
        conn: &MysqlConnection,
        id: Vec<u8>,
        curr_progress: i32,
    ) -> QueryResult<usize> {
        diesel::update(all_bookmarks::bookmarks.filter(all_bookmarks::bookmark_id.eq_all(id)))
            .set(all_bookmarks::ended_on.eq_all(curr_progress))
            .execute(conn)
    }
}
