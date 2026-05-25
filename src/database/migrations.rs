use sqlx::SqlitePool;

pub async fn migrate(
    pool: &SqlitePool,
) -> anyhow::Result<()> {

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS session (
            id            INTEGER PRIMARY KEY,
            token         TEXT NOT NULL,
            refresh_token TEXT NOT NULL DEFAULT '',
            role          TEXT NOT NULL DEFAULT 'student'
        )
        "#,
    )
    .execute(pool)
    .await?;

    let _ = sqlx::query(
        "ALTER TABLE session ADD COLUMN refresh_token TEXT NOT NULL DEFAULT ''",
    )
    .execute(pool)
    .await;

    let _ = sqlx::query(
        "ALTER TABLE session ADD COLUMN role TEXT NOT NULL DEFAULT 'student'",
    )
    .execute(pool)
    .await;

    let _ = sqlx::query(
        "ALTER TABLE session ADD COLUMN email TEXT NOT NULL DEFAULT ''",
    )
    .execute(pool)
    .await;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS classes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
