use sqlx::SqlitePool;

use crate::{
    api::{
        auth::login,
        client::ApiClient,
    },
    database::session_repository,
    models::auth::Session,
};

pub async fn authenticate(
    api_url: &str,
    pool: &SqlitePool,
    email: String,
    password: String,
) -> anyhow::Result<Session> {

    let api = ApiClient::new(
        api_url,
        None,
    );

    let response = login(
        &api,
        email,
        password,
    )
    .await?;

    session_repository::save_session(
        pool,
        &response.access_token,
        &response.refresh_token,
    )
    .await?;

    Ok(
        Session {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
        },
    )
}
