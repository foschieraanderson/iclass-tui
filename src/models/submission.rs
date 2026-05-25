use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Submission {
    pub id:           String,
    pub task_id:      String,
    pub student:      SubmissionStudent,
    pub file_url:     Option<String>,
    #[serde(rename = "textAnswer")]
    pub content:      Option<String>,
    pub grade:        Option<u32>,
    pub feedback:     Option<String>,
    pub graded_at:    Option<String>,
    #[serde(rename = "createdAt")]
    pub submitted_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmissionStudent {
    pub id:    String,
    pub name:  String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GradeSubmissionRequest {
    pub grade:    u32,
    pub feedback: Option<String>,
}
