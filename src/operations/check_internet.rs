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
        match response {
            Ok(response) => {
                format!("{},{}", &self.url, response.status().as_u16())
            }
            Err(e) => {
                format!("{},{}", &self.url, e)
            }
        }
    }
}