use aws_config::{from_env, SdkConfig};
use aws_credential_types::cache::CredentialsCache;

#[derive(Default)]
pub struct ConfigProvider {}

impl ConfigProvider {
    pub async fn provide(&self) -> SdkConfig {
        from_env()
            .credentials_cache(CredentialsCache::lazy())
            .load()
            .await
    }
}
