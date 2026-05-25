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
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
