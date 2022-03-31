use crate::entity::{prelude::*, todos};
use sea_orm::{EntityTrait};
use super::center::Repository;


impl Repository {
    pub async fn get_todos(&self) -> Vec<todos::Model> {
        Todos::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all todos")
    }
}