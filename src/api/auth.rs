use crate::{
    api::client::ApiClient,
    models::auth::{
        LoginRequest,
        LoginResponse,
    },
};

pub async fn login(
    api: &ApiClient,
    email: String,
    password: String,
) -> anyhow::Result<LoginResponse> {

    api.post(
        "/auth/login",
        &LoginRequest {
            email,
            password,
        },
    )
    .await
}
