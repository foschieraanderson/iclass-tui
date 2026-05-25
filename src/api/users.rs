use crate::{
    api::client::ApiClient,
    models::user::{
        CreateUserRequest,
        UpdateUserRequest,
        User,
    },
};

pub async fn list_users(
    api: &ApiClient,
) -> anyhow::Result<Vec<User>> {

    api.get("/users").await
}

pub async fn create_user(
    api: &ApiClient,
    req: CreateUserRequest,
) -> anyhow::Result<User> {

    api.post("/users", &req).await
}

pub async fn update_user(
    api: &ApiClient,
    id: &str,
    req: UpdateUserRequest,
) -> anyhow::Result<User> {

    api.patch(
        &format!("/users/{}", id),
        &req,
    )
    .await
}

pub async fn delete_user(
    api: &ApiClient,
    id: &str,
) -> anyhow::Result<()> {

    api.delete(&format!("/users/{}", id)).await
}

/// Lista usuários filtrados por role: `"teacher"` ou `"student"`.
/// Usa `GET /users?role=<role>` — filtro suportado pela API.
pub async fn list_users_by_role(
    api: &ApiClient,
    role: &str,
) -> anyhow::Result<Vec<User>> {

    api.get(&format!("/users?role={}", role)).await
}
