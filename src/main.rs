use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

#[macro_use]
extern crate diesel;

use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use models::User;

mod models;
mod schemas;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>) -> impl Responder {
    let conn = pool.get().expect("DB error");
    let user = web::block(move || User::get_all(&conn)).await.unwrap();
    HttpResponse::Ok().json2(&user)
}

#[get("/u/{u_id}")]
async fn by_id(pool: web::Data<DBPool>, u_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("DB error");
    let user = User::get_by_id(&conn, u_id.0).unwrap();
    HttpResponse::Ok().json2(&user[0])
}

#[post("/insert")]
async fn insert(pool: web::Data<DBPool>, user: Json<Vec<models::User>>) -> impl Responder {
    let conn = pool.get().expect("DB err");
    User::insert(&conn, user.0).expect("error while inserting");
    HttpResponse::Ok()
}

const PORT: u16 = 3000;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root@localhost:3306/test_db";
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
            .service(Files::new("/static", "static").show_files_listing())
            .service(index)
            .service(by_id)
            .service(insert)
    })
    .bind(format!("localhost:{}", PORT))?
    .run()
    .await
}
