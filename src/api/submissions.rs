use anyhow::Result;

use crate::{
    api::client::ApiClient,
    models::submission::{GradeSubmissionRequest, Submission},
};

pub async fn list_submissions(api: &ApiClient, task_id: &str) -> Result<Vec<Submission>> {
    api.get(&format!("/tasks/{}/submissions", task_id)).await
}

pub async fn submit(
    api: &ApiClient,
    task_id: &str,
    content: Option<String>,
    file_path: Option<String>,
) -> Result<Submission> {
    let mut fields = vec![];
    if let Some(c) = content {
        if !c.is_empty() {
            fields.push(("content".to_string(), c));
        }
    }
    api.post_form_with_file(&format!("/tasks/{}/submissions", task_id), fields, file_path).await
}

pub async fn grade_submission(
    api: &ApiClient,
    submission_id: &str,
    req: GradeSubmissionRequest,
) -> Result<Submission> {
    api.patch(&format!("/submissions/{}", submission_id), &req).await
}
