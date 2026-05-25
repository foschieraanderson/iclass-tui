pub struct Config {
    pub api_url: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            api_url: String::from(
                "http://localhost:3000/api/v1"
            ),

            database_url: String::from(
                "sqlite://cache.db"
            ),
        }
    }
}
