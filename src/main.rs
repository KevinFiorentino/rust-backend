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
    use self::models::{Post, NewPost, PostSimplificado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    // Lectura de variables de entorno
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    // Conexión con la BBDD
    let conn = PgConnection::establish(&db_url).expect("No se ha podido establecer la conexión con la base de datos.");

    // Insertamos el primer registro en la BBDD
    /* let new_post = NewPost {
        title: "Mi segundo post",
        body: "Lorem ipsum...",
        slug: "segundo-post",
    };
    diesel::insert_into(posts::table).values(new_post).get_result::<Post>(&conn).expect("Falló el insert en la BBDD"); */

    // SELECT * FROM posts;
    let posts_result = posts.load::<Post>(&conn).expect("Error en la consulta SQL.");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{}", post.title);
    }

    // SELECT * FROM posts LIMIT 1
    println!("Query con limites");
    let posts_result = posts.limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // SELECT * FROM posts ORDER BY id LIMIT 1
    println!("Query con limites ordenado por id");
    let posts_result = posts.order(id.desc()).limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


    // SELECT title, body FROM posts LIMIT 1
    println!("Query columnas especificas");
    let posts_result = posts.select((title, body)).limit(1).load::<PostSimplificado>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


    // SELECT title, body FROM posts WHERE id = 2 LIMIT 1
    println!("Query con where");
    let posts_result = posts.filter(slug.eq("primer-post")).limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }

}

