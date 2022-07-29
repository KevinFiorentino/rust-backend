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

// Importamos TERA
use tera::Tera;


/* ********************
          GET
******************** */

#[get("/tera_test")]
async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {

    // Creamos un contexto para pasarle datos al template
    let mut ctx = tera::Context::new();

    // Enviamos el template que queremos localizándolo por su nombre
    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("tera_test.html", &ctx).unwrap()
    )
}

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    // Consulta para obtener todos los registros
    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            // Enviamos, a través del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            // Pasamos los datos al template index.html
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

// Capturamos el parámetro por URL
#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>, 
    template_manager: web::Data<tera::Tera>, 
    blog_slug: web::Path<String>
) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            // Si el post no existe, devolvemos 404
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            // Enviamos, a través del contexto, los datos del post al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("post.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}


/* ********************
          POST
******************** */

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


/* ********************
          MAIN
******************** */

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");
    let port = env::var("PORT").expect("La variable de entorno PORT no existe.");
    let port: u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool.");

    HttpServer::new(move || {
        
        // Instanciamos TERA y le indicamos en qué directorio buscar los templates
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .service(tera_test)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    }).bind(("0.0.0.0", port)).unwrap().run().await
}
