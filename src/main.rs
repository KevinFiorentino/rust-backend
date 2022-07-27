#[macro_use]

extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts::dsl::*;


#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let conn = pool.get().expect("Problemas al traer el pool de conexión.");

    // El 'match' responde en caso de éxito o error en la consulta
    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir los datos.")
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let conn = pool.get().expect("Problemas al traer la base de datos");

    // Utiliamos la función creada en el modelo para crear un nuevo registro y devolverlo
    match web::block(move || {Post::create_post(&conn, &item)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El POOL sirve para compartir la conexión con otros servicios
    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool.");

    HttpServer::new(move || {
        // Compartimos el pool de conexión a cada endpoint
        App::new()
            .service(index)
            .service(new_post)
            .app_data(web::Data::new(pool.clone()))
    }).bind(("127.0.0.1", 8080)).unwrap().run().await
}
