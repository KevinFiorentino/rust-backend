// Macro para indicar que los registros de la BBDD tendrán la misma forma que la estructura.
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}