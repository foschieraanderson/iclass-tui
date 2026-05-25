use serde::{
    Deserialize,
    Serialize,
};

use crate::models::user::User;

// ---- API response ---------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassRoom {
    pub id: String,
    pub code: String,
    pub period: String,
    pub grade: String,
    pub teacher: User,
    pub students: Vec<User>,
}

// ---- Requests -------------------------------------------------

/// POST /api/v1/classes — admin only
/// period: "YYYY/1" ou "YYYY/2"
/// grade: "3A", "5B", etc. (API transforma para maiúsculas)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassRequest {
    pub period: String,
    pub grade: String,
    pub teacher_id: String,
    pub student_ids: Vec<String>,
}

/// PATCH /api/v1/classes/:id — admin only
/// Todos os campos são opcionais; ao menos um deve ser enviado.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClassRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student_ids: Option<Vec<String>>,
}
