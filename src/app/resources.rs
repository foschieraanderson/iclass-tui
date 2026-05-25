#[derive(Debug, Clone)]
pub enum Resource<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}
