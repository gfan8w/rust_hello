#![allow(unused)]

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

#[derive(Debug,FromRow)]
struct Ticket{
    id: i64,
    name: String,
}


//#[tokio::main]
#[actix_web::main]
async fn main () -> Result<(), sqlx::Error>{
    //1）创建连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/graphql_todos_example")
        .await?;

    //2）创建表，如果表不存在
    sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS ticket (
  id bigserial,
  name text
);
    "#)
        .execute(&pool)
        .await?;

    //3）插入新数据
    let row:(i64,) =sqlx::query_as("insert into ticket (name) values ($1) returning id")
        .bind("a new ticket")
        .fetch_one(&pool)
        .await?;

    println!("{:?}",row);

    //4)查询所有
    let allrows= sqlx::query("select * from ticket").fetch_all(&pool).await?;
    let allrows_txt = allrows.iter()
        .map(|r|{
            format!("{} - {}", r.get::<i64,_>("id"), r.get::<String,_>("name"))
        })
        .collect::<Vec<_>>()
        .join(", ");

    println!("allrows:{}",allrows_txt);


    // 5) 使用map 构建自定义对象
    let select_query = sqlx::query(" select * from ticket");
    let tickets = select_query
        .map(|row: PgRow|{
            Ticket {
                id:row.get("id"),
                name:row.get("name")
            }
        })
        .fetch_all(&pool)
        .await?;

    println!("\n=== select tickets with query.map...:\n{:?}", tickets);

    //6) 使用query_as，使用 derive里的FromRow自动构建对象
    let select_query =sqlx::query_as::<_,Ticket>("select id,name from ticket");
    let ticks = select_query.fetch_all(&pool).await?;
    println!("\n=== select tickets with query_as FromRow...: \n{:?}", ticks);

    Ok(())


}