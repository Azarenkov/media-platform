use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn db_connect(url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(url)
        .await
        .expect("Postgres connection error");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Postgres migrations error");

    pool
}
