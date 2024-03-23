use async_trait::async_trait;

use crate::operations::calculate::Calculate;

pub struct CheckInternet {
    url: String,
}

impl CheckInternet {
    pub fn new(url: String) -> Self {
        CheckInternet { url }
    }
}

#[async_trait]
impl Calculate for CheckInternet {
    async fn calculate(&self) -> String {
        let response = reqwest::get(&self.url).await;
        let status = match response {
            Ok(response) => {
                let binding = response.status();
                binding.to_string()
            }
            Err(e) => {
                let binding = e.status();
                binding.map(|s| s.to_string()).unwrap_or_else(|| "-1".to_string())
            }
        };
        format!("\"{}\",\"{}\"", &self.url, status)
    }
}