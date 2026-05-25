use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassReport {
    pub class_id:   String,
    pub class_code: String,
    pub tasks:      Vec<ReportTask>,
    pub students:   Vec<ReportStudent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReportTask {
    pub id:    String,
    pub title: String,
    pub score: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportStudent {
    pub id:             String,
    pub name:           String,
    pub email:          String,
    pub total_earned:   u32,
    pub total_possible: u32,
}
