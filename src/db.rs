use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn connect() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos")
}