use diesel::Queryable;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct ToDo {
    pub id: i32,
    pub title: String,
    pub completed: bool
}