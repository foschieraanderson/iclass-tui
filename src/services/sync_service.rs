use sqlx::SqlitePool;

use crate::api::client::ApiClient;

pub async fn sync_all(
    _api: &ApiClient,
    _pool: &SqlitePool,
) -> anyhow::Result<()> {

    /*
       1. busca API
       2. salva sqlite
       3. atualiza store
    */

    Ok(())
}
