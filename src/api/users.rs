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

    api.get("/api/v1/users").await
}

pub async fn create_user(
    api: &ApiClient,
    req: CreateUserRequest,
) -> anyhow::Result<User> {

    api.post("/api/v1/users", &req).await
}

pub async fn update_user(
    api: &ApiClient,
    id: i64,
    req: UpdateUserRequest,
) -> anyhow::Result<User> {

    api.patch(
        &format!("/api/v1/users/{}", id),
        &req,
    )
    .await
}

pub async fn delete_user(
    api: &ApiClient,
    id: i64,
) -> anyhow::Result<()> {

    api.delete(&format!("/api/v1/users/{}", id)).await
}
