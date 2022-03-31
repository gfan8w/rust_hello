use diesel::{Queryable,Insertable};
use crate::schema::todos;  //引入 todos 表的定义

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

///定义一个插入数据的结构体
/// juniper::GraphQLInputObject: 告诉GraphQL这是一个插入的schema.
/// Insertable 告诉diesel，这个是可以插入的
/// table_name 告诉 diesel，保存到哪个表
#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub completed: bool
}