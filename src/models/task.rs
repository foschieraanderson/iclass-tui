use serde::Deserialize;

use crate::models::user::User;

/// Referência resumida da turma embutida na resposta de task.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassRef {
    pub id: String,
    pub code: String,
    pub period: String,
    pub grade: String,
}

/// Task retornada pela API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub class_id: String,
    pub title: String,
    pub description: Option<String>,
    pub file_url: Option<String>,
    pub score: i32,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub class: ClassRef,
    pub created_by: User,
}
