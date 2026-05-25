use anyhow::Result;

use crate::{api::client::ApiClient, models::report::ClassReport};

pub async fn get_class_report(api: &ApiClient, class_id: &str) -> Result<ClassReport> {
    api.get(&format!("/classes/{}/report", class_id)).await
}
