use crate::error::PermanentError;
use aws_config::SdkConfig;

pub struct UpdateService {}

impl UpdateService {
    pub fn new(_config: &SdkConfig) -> Self {
        UpdateService {}
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_firmware_update(
        &self,
        current_version: String,
    ) -> Result<Option<String>, PermanentError> {
        Ok(Some("update available".to_string()))
    }
}
