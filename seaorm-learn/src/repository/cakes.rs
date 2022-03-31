use crate::entity::{prelude::*, cake};
use sea_orm::{ColumnTrait, DbBackend, DeleteResult, EntityTrait, FromQueryResult, NotSet, QueryFilter, QueryOrder, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::ColumnSpec::Default;
use crate::Repository;
use sea_orm::ActiveModelTrait;



impl Repository {
    pub async fn get_cake(&self, cake_id: i32) -> Vec<cake::Model> {

        let yellow ="yellow cake".to_string();

        // delete if exist
        let res: DeleteResult =cake::Entity::delete_many()
            .filter(cake::Column::Name.eq(yellow.clone()))
            .exec(&self.db_conn)
            .await
            .unwrap();

        println!("delete yellow cake:{}",res.rows_affected);

        // insert an object
        let yellow_cake = cake::ActiveModel{
            name: Set(yellow.clone()),
            desc:NotSet,
            id: NotSet
            //..Default::default()
        };

        // save to db,then retrieve again
        let inserted_yellow= yellow_cake.save(&self.db_conn).await;
        println!("inserted yellow:{:?}",inserted_yellow);

        //convert to ActiveModel, modify a field,then save to db again
        let mut update_yellow: cake::ActiveModel = inserted_yellow.unwrap().into();
        update_yellow.desc=Set(Some("yellow cake come from yellow county".to_string()));

        // save to db, update a already one
        let after_update_yellow = update_yellow.update(&self.db_conn).await;
        println!("after update yellow cake:{:?}", after_update_yellow);

        // query by filter
        let choco: Vec<cake::Model> = Cake::find().filter(cake::Column::Name.contains("choco"))
            .order_by_desc(cake::Column::Id)
            .all(&self.db_conn)
            .await
            .map_err(|e| format!("db error:{}",e.to_string()))
            .expect("error get chocolate cakes");

        println!("chocolate cakes:{:?}", choco);


        // use raw sql
        let cheese: Option<cake::Model> = cake::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(DbBackend::MySql,
            r#"select cake.id, cake.name, cake.desc from cake where cake.id =? "#,
                vec![1.into()]
        ))
            .one(&self.db_conn)
            .await.unwrap();
        println!("cheese cakes:{:?}", cheese);


        // use raw sql, return a customised entity
        let mycake: Vec<MyCake> = MyCake::find_by_statement
            (Statement::from_sql_and_values(DbBackend::MySql,
                                                         r#"select cake.name, cake.desc from cake where cake.id>? "#,
                                                         vec![1.into()]
            ))
            .all(&self.db_conn)
            .await.unwrap();
        println!("my cakes:{:?}", mycake);



        Cake::find_by_id(cake_id)
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all todos")
    }
}



#[derive(Debug, FromQueryResult)]
struct MyCake {
    pub name: String,
    pub desc: Option<String>,
}