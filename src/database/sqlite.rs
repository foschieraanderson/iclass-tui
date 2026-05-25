use std::str::FromStr;

use sqlx::{
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    SqlitePool,
};

pub async fn connect(
    database_url: &str,
) -> anyhow::Result<SqlitePool> {

    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}
