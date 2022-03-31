use sea_orm::{DatabaseConnection, EntityTrait};


#[derive(Debug, Clone)]
pub struct Repository {
    pub db_conn: DatabaseConnection,
}