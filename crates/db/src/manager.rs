use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

pub struct Manager {
    pub pool: crate::Pool,
}

impl Manager {
    pub async fn new() -> anyhow::Result<Self> {
        dotenvy::dotenv().context("Failed to load dotenv")?;
        let url = dotenvy::var("DATABASE_URL")
            .context("Environment variable 'DATABASE_URL' has to be set")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .context("Failed to open database pool")?;

        Ok(Self { pool })
    }
}
