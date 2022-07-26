#[macro_use]

extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

// Librerías para crear una conexión a la BBDD y compartirla en toda la aplicación
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;


#[get("/")]
async fn hello_wold() -> impl Responder {
    HttpResponse::Ok().body("Hola Platzi!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El POOL sirve para compartir la conexión con otros servicios
    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool");

    HttpServer::new(move || {
        // Compartimos el pool de conexión a cada endpoint
        App::new().service(hello_wold).app_data(pool.clone())       
    }).bind(("127.0.0.1", 8080)).unwrap().run().await

}
