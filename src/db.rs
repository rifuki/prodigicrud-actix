use sqlx::mysql::{
    MySqlPool,
    MySqlPoolOptions
};

pub type DbPool = MySqlPool;

pub async fn establish_connection(db_url: &str) -> DbPool {
    MySqlPoolOptions::new()
        .connect(db_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed Create Database Pool. [{}]", e);
            std::process::exit(1);
        })
}   