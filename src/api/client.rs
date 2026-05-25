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

    pub async fn patch<B, T>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T>
    where
        B: serde::Serialize,
        T: serde::de::DeserializeOwned,
    {
        let response = self.client
            .patch(format!(
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

    /// POST com multipart/form-data (sem arquivo — apenas campos de texto).
    pub async fn post_form<T>(
        &self,
        path: &str,
        fields: Vec<(String, String)>,
    ) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let form = fields
            .into_iter()
            .fold(reqwest::multipart::Form::new(), |f, (k, v)| f.text(k, v));

        let response = self.client
            .post(format!("{}{}", self.base_url, path))
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<T>().await?)
    }

    /// PATCH com multipart/form-data (sem arquivo — apenas campos de texto).
    pub async fn patch_form<T>(
        &self,
        path: &str,
        fields: Vec<(String, String)>,
    ) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let form = fields
            .into_iter()
            .fold(reqwest::multipart::Form::new(), |f, (k, v)| f.text(k, v));

        let response = self.client
            .patch(format!("{}{}", self.base_url, path))
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<T>().await?)
    }

    pub async fn delete(
        &self,
        path: &str,
    ) -> anyhow::Result<()> {

        self.client
            .delete(format!(
                "{}{}",
                self.base_url,
                path,
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
