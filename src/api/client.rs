use reqwest::{
    header,
    Client,
};

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {

    pub fn new(
        base_url: impl Into<String>,
        token: Option<String>,
    ) -> Self {

        let mut headers = header::HeaderMap::new();

        if let Some(token) = token {

            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", token)
                    .parse()
                    .unwrap(),
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    pub async fn get<T>(
        &self,
        path: &str,
    ) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.client
            .get(format!(
                "{}{}",
                self.base_url,
                path,
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<T>().await?)
    }

    pub async fn post<B, T>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T>
    where
        B: serde::Serialize,
        T: serde::de::DeserializeOwned,
    {
        let response = self.client
            .post(format!(
                "{}{}",
                self.base_url,
                path,
            ))
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<T>().await?)
    }
}
