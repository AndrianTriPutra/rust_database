use sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: &str) -> DatabaseConnection {
    Database::connect(url)
        .await
        .expect("Failed to connect to the database") 
}
