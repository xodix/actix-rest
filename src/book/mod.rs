mod book_schema;

use book_schema::books;
use books::dsl as all_books;

use diesel::{EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
pub struct Book {
    pub book_id: Vec<u8>,
    pub title: String,
    pub author: Vec<u8>,
    pub description: String,
    pub tags: String,
    pub cover_url: String,
}

impl Book {
    pub fn new(
        title: String,
        author: Vec<u8>,
        description: String,
        tags: String,
        cover_url: String,
    ) -> Self {
        let book_id = uuid::Uuid::new_v4().as_bytes().to_vec();
        Self {
            book_id,
            title,
            author,
            description,
            tags,
            cover_url,
        }
    }

    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Book>> {
        books::table.load(conn)
    }

    pub fn insert(conn: &MysqlConnection, vals: Vec<Book>) -> QueryResult<usize> {
        diesel::insert_into(books::table).values(vals).execute(conn)
    }

    pub fn remove(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<usize> {
        diesel::delete(all_books::books.filter(all_books::book_id.eq_all(id))).execute(conn)
    }

    pub fn get_by_id(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<Vec<Book>> {
        all_books::books
            .filter(all_books::book_id.eq_all(id))
            .load(conn)
    }

    pub fn change_title(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_titile: String,
    ) -> QueryResult<usize> {
        diesel::update(all_books::books.filter(all_books::book_id.eq_all(id)))
            .set(all_books::title.eq_all(new_titile))
            .execute(conn)
    }

    pub fn change_description(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_description: String,
    ) -> QueryResult<usize> {
        diesel::update(all_books::books.filter(all_books::book_id.eq_all(id)))
            .set(all_books::description.eq_all(new_description))
            .execute(conn)
    }

    pub fn add_tag(conn: &MysqlConnection, id: Vec<u8>, new_tag: &str) -> QueryResult<usize> {
        let query: QueryResult<Vec<Book>> = all_books::books
            .filter(all_books::book_id.eq_all(&id))
            .load(conn);
        let result: Vec<Book> = query.ok().unwrap();

        let tags: String = result[0].tags.clone() + " " + new_tag;

        diesel::update(all_books::books.filter(all_books::book_id.eq_all(id)))
            .set(all_books::tags.eq_all(tags))
            .execute(conn)
    }

    pub fn remove_tag(
        conn: &MysqlConnection,
        id: Vec<u8>,
        unnecesery_tag: &str,
    ) -> QueryResult<usize> {
        let query: QueryResult<Vec<Book>> = all_books::books
            .filter(all_books::book_id.eq_all(&id))
            .load(conn);
        let result: Vec<Book> = query.ok().unwrap();

        let mut tags: String = result[0].tags.replace(unnecesery_tag, "");
        tags = tags.replace("  ", " ");
        diesel::update(all_books::books.filter(all_books::book_id.eq_all(id)))
            .set(all_books::tags.eq_all(tags))
            .execute(conn)
    }

    pub fn change_cover(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_cover: &str,
    ) -> QueryResult<usize> {
        diesel::update(all_books::books.filter(all_books::book_id.eq_all(id)))
            .set(all_books::cover_url.eq_all(new_cover))
            .execute(conn)
    }
}
