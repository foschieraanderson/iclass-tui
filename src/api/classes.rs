use crate::{
    api::client::ApiClient,
    models::class::{
        ClassRoom,
        CreateClassRequest,
        UpdateClassRequest,
    },
};

/// GET /classes — todos os usuários autenticados
pub async fn list_classes(
    api: &ApiClient,
) -> anyhow::Result<Vec<ClassRoom>> {
    api.get("/classes").await
}

/// POST /classes — somente admin
pub async fn create_class(
    api: &ApiClient,
    req: CreateClassRequest,
) -> anyhow::Result<ClassRoom> {
    api.post("/classes", &req).await
}

/// PATCH /classes/:id — somente admin
pub async fn update_class(
    api: &ApiClient,
    id: &str,
    req: UpdateClassRequest,
) -> anyhow::Result<ClassRoom> {
    api.patch(&format!("/classes/{}", id), &req).await
}

/// DELETE /classes/:id — somente admin
pub async fn delete_class(
    api: &ApiClient,
    id: &str,
) -> anyhow::Result<()> {
    api.delete(&format!("/classes/{}", id)).await
}
