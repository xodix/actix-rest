use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::ContentEncoding;
use actix_web::{
    get, middleware,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use std::env;

#[macro_use]
extern crate diesel;

use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};

mod book;
mod book_content;
mod bookmark;
mod edit;
mod user;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>) -> impl Responder {
    let conn = pool.get().expect("DB err");
    HttpResponse::Ok().json(book::Book::get_all(&conn).unwrap())
}

#[get("/insert")]
async fn book_insert(pool: web::Data<DBPool>) -> impl Responder {
    use uuid::Uuid;
    let conn = pool.get().expect("DB err");
    let vals = vec![book::Book::new(
        "Nice".to_string(),
        Uuid::new_v4().as_bytes().to_vec(),
        "not good".to_string(),
        Uuid::new_v4().as_bytes().to_vec(),
        "yes very good".to_string(),
        "https://fuck.you/jd.jpg".to_string(),
    )];
    let message = book::Book::insert(&conn, vals).expect("Insert error:");
    HttpResponse::Ok().body(format!("oki: {}", message))
}

#[get("/insert")]
async fn book_content_insert(pool: web::Data<DBPool>) -> impl Responder {
    use uuid::Uuid;
    let conn = pool.get().expect("DB err");
    let vals = vec![book_content::BookContent::new(
        Uuid::new_v4().as_bytes().to_vec(),
        "## nice".to_string(),
        Uuid::new_v4().as_bytes().to_vec(),
    )];
    let message = book_content::BookContent::insert(&conn, vals).expect("Insert error");
    HttpResponse::Ok().body(format!("oki: {}", message))
}

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let url = env::var("DB_URL").expect("Cannot read db url from .env");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to make a r2d2 pool");
    println!(
        "Server listening on http://localhost:{}\nCTRL+C to close",
        PORT
    );
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::permissive())
            .service(Files::new("/static", "static").show_files_listing())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(index)
            .service(web::scope("/book").service(book_insert))
            .service(web::scope("/book_content").service(book_content_insert))
    })
    .bind(format!("localhost:{}", PORT))?
    .run()
    .await
}
