#![allow(unused)]

use std::ops::Sub;
use chrono::{Duration, Local, TimeZone, Utc};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{Executor, FromRow, Row};

#[derive(Debug,FromRow)]
struct Book{
    id: i64,
    name: String,
    author: Option<String>,
    created_at:chrono::NaiveDateTime,
    updated_at:chrono::NaiveDateTime,
}


//#[tokio::main]
#[actix_web::main]
async fn main () -> Result<(), sqlx::Error>{
    //1）创建连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .after_connect(|conn| Box::pin(async move {
                    conn.execute("SET time_zone = SYSTEM;").await?; //修正时区，参考：https://github.com/launchbadge/sqlx/issues/263
                   Ok(())
                 }))
        .connect("mysql://root:123456@localhost/actix_learn")
        .await?;



    //2）创建表，如果表不存在
    //let create_sql =include_str!("create.sql");
    let create_sql ="CREATE TABLE IF NOT EXISTS book( id int NOT NULL PRIMARY KEY AUTO_INCREMENT, name varchar(255) NOT NULL, author varchar(255), created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP ) ENGINE=INNODB CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci; ";
    sqlx::query(create_sql)
        .execute(&pool)
        .await?;

    //3）插入新数据
    let mut book = Book {
        id: 0,
        name: String::from("Rust 實踐"),
        author: Some(String::from("無名氏")),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local()
    };
    book.created_at=book.created_at-Duration::days(1);

    println!("book:{:?}",book);

    //开启一个事务，获取最新插入的数据的 自增长id
    {
        let mut tx_conn = pool.begin().await?;
        sqlx::query("insert into book (name,author,created_at) values (?,?,?);")
            .bind(book.name)
            .bind(book.author.unwrap())
            .bind(book.created_at)
            .execute(&mut tx_conn)
            .await?;
        //.last_insert_id();

        println!("insert done");

        let last_book_Id = sqlx::query_as::<_, (u32, )>("SELECT LAST_INSERT_ID() as id;")
            .fetch_one(&mut tx_conn)
            .await?;
        println!("last insert id:{:?}", last_book_Id.0);

        tx_conn.commit().await?
    }

    let book = sqlx::query(r#"select * from book where id =?"#)
        .bind(1)
        .fetch_one(&pool)
        .await?;
    println!("book just inserted:{:?}",book);

    //4)查询所有
    let allrows= sqlx::query("select * from book").fetch_all(&pool).await?;
    let allrows_txt = allrows.iter()
        .map(|r|{
            format!("{} - {} , authoer:{}", r.get::<i64,_>("id"), r.get::<String,_>("name"), r.get::<String,_>("author"))
        })
        .collect::<Vec<_>>()
        .join(",  ");

    println!("allrows:{}",allrows_txt);


    // 5) 使用map 构建自定义对象
    let select_query = sqlx::query(" select * from book");
    let tickets = select_query
        .map(|row: MySqlRow|{
            Book {
                id:row.get("id"),
                name:row.get("name"),
                author: row.get("author"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        })
        .fetch_all(&pool)
        .await?;

    println!("\n=== select books with query.map...:\n");
    for x in tickets {
        println!("book:{:?}",x)
    }

    //6) 使用query_as，使用 derive里的FromRow自动构建对象
    let select_query =sqlx::query_as::<_,Book>("select id,name,updated_at,created_at,author from book");
    let ticks = select_query.fetch_all(&pool).await?;
    println!("\n=== select books with query_as FromRow...: \n");
    for x in ticks {
        println!("book:{:?}",x);
        let lt =Local.from_local_datetime(&x.created_at);
        println!("local time:{:?}",lt);

        let lt =Local.from_utc_datetime(&x.created_at);
        println!("local to utc time:{:?}",lt);

        let lt =Utc.from_local_datetime(&x.created_at);
        println!("utc local time:{:?}",lt);

        let lt =Utc.from_utc_datetime(&x.created_at);
        println!("utc time:{:?}",lt)
    }
    Ok(())


}