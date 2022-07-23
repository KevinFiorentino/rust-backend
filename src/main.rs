// Indicamos que vamos a utilizar macros
#[macro_use]

// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::prelude::*;
use diesel::pg::PgConnection;

// Importamos librerias para leer variables de entorno
use dotenv::dotenv;
use std::env;

// Importamos esquemas de BBDD y modelos
pub mod schema;
pub mod models;

fn main() {

    // Indicamos que vamos a utilizar el esquema de Posts y el modelo
    use self::models::Post;
    use self::schema::posts::dsl::*;

    // Lectura de variables de entorno
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    // Conexión con la BBDD
    let conn = PgConnection::establish(&db_url).expect("No se ha podido establecer la conexión con la base de datos.");

    // Realizamos la consulta equivalente a: SELECT * FROM posts;
    let posts_result = posts.load::<Post>(&conn).expect("Error en la consulta SQL.");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{}", post.title);
    }

}
