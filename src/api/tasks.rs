use crate::{
    api::client::ApiClient,
    models::task::Task,
};

/// GET /tasks — retorna tarefas visíveis ao usuário autenticado.
pub async fn list_tasks(api: &ApiClient) -> anyhow::Result<Vec<Task>> {
    api.get("/tasks").await
}

/// POST /tasks — admin | teacher (multipart, sem arquivo).
pub async fn create_task(
    api: &ApiClient,
    fields: Vec<(String, String)>,
) -> anyhow::Result<Task> {
    api.post_form("/tasks", fields).await
}

/// PATCH /tasks/:id — admin | teacher (multipart, sem arquivo).
pub async fn update_task(
    api: &ApiClient,
    id: &str,
    fields: Vec<(String, String)>,
) -> anyhow::Result<Task> {
    api.patch_form(&format!("/tasks/{}", id), fields).await
}

/// DELETE /tasks/:id — admin | teacher. Retorna 204 sem corpo.
pub async fn delete_task(api: &ApiClient, id: &str) -> anyhow::Result<()> {
    api.delete(&format!("/tasks/{}", id)).await
}
