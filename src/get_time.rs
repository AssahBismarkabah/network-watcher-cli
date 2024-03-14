use async_trait::async_trait;
use chrono::Local;
use crate::calculate::Calculate;

pub struct GetTime;

impl GetTime {
    pub fn new() -> Self {
        GetTime {}
    }

    pub fn get_time(&self) -> String {
        let now = Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[async_trait]
impl Calculate for GetTime {
    async fn calculate(&self) -> String {
        self.get_time()
    }
}
