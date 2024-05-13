use crate::error::PermanentError;
use aws_config::SdkConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use futures::stream::IntoStream;
use futures::TryStreamExt;

pub struct UpdateService {
    s3_client: Client,
}

impl UpdateService {
    pub fn new(config: &SdkConfig) -> Self {
        UpdateService {
            s3_client: Client::new(config),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_firmware_update(
        &self,
        update_file: String,
    ) -> Result<Option<IntoStream<ByteStream>>, PermanentError> {
        return Ok(Some(
            self.s3_client
                .get_object()
                .bucket("coriolis_firmware_updates")
                .key(update_file)
                .send()
                .await
                .unwrap()
                .body
                .into_stream(),
        ));
    }
}
