use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection(url: String) -> Result<DatabaseConnection, DbErr> {
    Database::connect(url).await
}
