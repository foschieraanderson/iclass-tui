use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    sqlx::FromRow,
)]
pub struct ClassRoom {
    pub id: i64,
    pub title: String,
}
