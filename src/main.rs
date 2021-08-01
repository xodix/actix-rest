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
mod schemas;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>) -> impl Responder {
    let conn = pool.get().expect("DB err");
    HttpResponse::Ok().body("{}")
}

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let url = env::var("DB_URL").expect("Cannot read db url");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to make a pool");
    println!(
        "Server listening on http://localhost:{}\nCTRL+C to close",
        PORT
    );
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(Files::new("/static", "static").show_files_listing())
            .service(index)
    })
    .bind(format!("localhost:{}", PORT))?
    .run()
    .await
}
