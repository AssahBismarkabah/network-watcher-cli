use async_trait::async_trait;

use crate::calculate::Calculate;

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
        let mut result = String::new();
        let response = reqwest::get(&self.url).await;
        match response {
            Ok(response) => {
                result.push_str(&format!("{},{}", &self.url, response.status()));
            }
            Err(e) => {
                result.push_str(&format!("{},{}", &self.url, e));
            }
        }
        result
    }
}