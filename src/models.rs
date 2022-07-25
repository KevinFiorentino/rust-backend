// Importamos el esquema de la BBDD
use super::schema::posts;

// Macro para indicar que los registros de la BBDD tendrán la misma forma que la estructura.
#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

// Macro para indicar que la estructura servirá que insert en la BBDD
#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

// Estructura para obtener registros parciales de la BBDD
#[derive(Queryable, Debug)]
pub struct PostSimplificado {
    pub title: String,
    pub body: String,
}