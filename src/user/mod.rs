mod user_schema;

use diesel::{EqAll, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use user_schema::users;
use user_schema::users::dsl as all_users;

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
pub struct User {
    user_id: Vec<u8>,
    name: String,
    email: String,
    phone_num: i32,
    img_url: String,
    password: String,
    favourites: String,
}

impl User {
    pub fn new(
        user_id: Vec<u8>,
        name: String,
        email: String,
        phone_num: i32,
        img_url: String,
        password: String,
        favourites: String,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            phone_num,
            img_url,
            password,
            favourites,
        }
    }

    pub fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
        all_users::users.load(conn)
    }

    pub fn get_by_id(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<Vec<User>> {
        all_users::users
            .filter(all_users::user_id.eq_all(id))
            .load(conn)
    }

    pub fn insert(conn: &MysqlConnection, vals: Vec<User>) -> QueryResult<usize> {
        diesel::insert_into(all_users::users)
            .values(vals)
            .execute(conn)
    }

    pub fn remove(conn: &MysqlConnection, id: Vec<u8>) -> QueryResult<usize> {
        diesel::delete(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .execute(conn)
    }

    pub fn change_email(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_name: String,
    ) -> QueryResult<usize> {
        diesel::update(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .set(all_users::email.eq_all(new_name))
            .execute(conn)
    }

    pub fn change_phone_number(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_pn: i32,
    ) -> QueryResult<usize> {
        diesel::update(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .set(all_users::phone_num.eq_all(new_pn))
            .execute(conn)
    }

    pub fn change_password(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_pass: String,
    ) -> QueryResult<usize> {
        diesel::update(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .set(all_users::password.eq_all(new_pass))
            .execute(conn)
    }

    pub fn add_favourite(
        conn: &MysqlConnection,
        id: Vec<u8>,
        new_fav: String,
    ) -> QueryResult<usize> {
        let query: QueryResult<Vec<User>> = all_users::users
            .filter(all_users::user_id.eq_all(&id))
            .load(conn);
        let result = query.ok().unwrap();

        let favs = result[0].favourites.clone();
        let new_favs = favs + " " + &new_fav;

        diesel::update(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .set(all_users::favourites.eq_all(new_favs))
            .execute(conn)
    }

    pub fn remove_favourite(
        conn: &MysqlConnection,
        id: Vec<u8>,
        unnecesery_fav: String,
    ) -> QueryResult<usize> {
        let query: QueryResult<Vec<User>> = all_users::users
            .filter(all_users::user_id.eq_all(&id))
            .load(conn);
        let result = query.ok().unwrap();

        let favs = result[0].favourites.clone().replace(&unnecesery_fav, "");
        let new_favs = favs.replace("  ", " ");
        diesel::update(all_users::users)
            .filter(all_users::user_id.eq_all(id))
            .set(all_users::favourites.eq_all(new_favs))
            .execute(conn)
    }
}
