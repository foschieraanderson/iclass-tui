use sqlx::SqlitePool;

use crate::models::auth::Session;

pub async fn save_session(
    pool: &SqlitePool,
    access_token: &str,
    refresh_token: &str,
    role: &str,
) -> anyhow::Result<()> {

    sqlx::query(
        r#"
        INSERT OR REPLACE INTO session (
            id,
            token,
            refresh_token,
            role
        )
        VALUES (1, ?, ?, ?)
        "#,
    )
    .bind(access_token)
    .bind(refresh_token)
    .bind(role)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_session(
    pool: &SqlitePool,
) -> anyhow::Result<()> {

    sqlx::query(
        "DELETE FROM session WHERE id = 1",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn load_session(
    pool: &SqlitePool,
) -> anyhow::Result<Option<Session>> {

    let row = sqlx::query_as::<_, (String, String, String)>(
        r#"
        SELECT token, refresh_token, role
        FROM session
        WHERE id = 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(
        row.map(|r| Session {
            access_token: r.0,
            refresh_token: r.1,
            role: r.2,
        }),
    )
}
