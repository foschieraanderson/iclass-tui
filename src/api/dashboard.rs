use anyhow::Result;

use crate::{
    api::client::ApiClient,
    models::dashboard::DashboardData,
};

pub async fn get_dashboard(api: &ApiClient) -> Result<DashboardData> {
    api.get("/dashboard").await
}
