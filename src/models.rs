use diesel::prelude::*;
use super::schema::posts;

// Permitimos a las estructuras manipular datos en formato JSON
use serde::{Deserialize, Serialize};

// Estructura para obtener los registros completos desde la BBDD
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

// Estructura para obtener solo algunos campos de los registros
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct PostSimplificado {
    pub title: String,
    pub body: String,
}

// Estructura para crear registros en la BBDD con el formato de un post
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

// Estrucutra para recibir desde un cliente datos en formato JSON
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String
}


impl Post {
    // Función para crear un String con el formato de un slug, minúscula y separado por guión medio
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    // Función para crear un nuevo Post en la BBDD a partir de datos de entrada
    pub fn create_post<'a> (
        conn: &PgConnection, 
        post: &NewPostHandler
    ) -> Result<Post, diesel::result::Error> {

        let slug = Post::slugify(&post.title.clone());

        let new_post = NewPost{
            title: &post.title,
            slug: &slug,
            body: &post.body
        };

        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }
}
