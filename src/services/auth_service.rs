use base64::{
    engine::general_purpose,
    Engine as _,
};

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

    let role = extract_role(&response.access_token);

    session_repository::save_session(
        pool,
        &response.access_token,
        &response.refresh_token,
        &role,
    )
    .await?;

    Ok(
        Session {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            role,
        },
    )
}

fn extract_role(token: &str) -> String {

    let payload = token
        .split('.')
        .nth(1)
        .and_then(|part| {
            general_purpose::URL_SAFE_NO_PAD
                .decode(part)
                .ok()
        })
        .and_then(|bytes| {
            serde_json::from_slice::<serde_json::Value>(&bytes).ok()
        })
        .and_then(|json| {
            json.get("role")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

    payload.unwrap_or_else(|| "student".to_string())
}
