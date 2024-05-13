use crate::error::PermanentError;
use crate::service::UpdateService;
use actix_web::web::{Data, Query};
use lambda_web::actix_web::{get, HttpResponse};
use serde::Deserialize;
use tracing::info;
use utoipa::IntoParams;

#[derive(Deserialize, Debug, IntoParams)]
pub struct FirmwareUpdateParams {
    pub current_version: String,
}

#[utoipa::path(
    params(
    FirmwareUpdateParams
    ),
    responses(
        (status = 200, description = "Return if a firmware update is available")
    )
)]
#[tracing::instrument(skip(update_service))]
#[get("/operational/firmware_update")]
pub async fn firmware_update(
    update_service: Data<UpdateService>,
    params: Query<FirmwareUpdateParams>,
) -> Result<HttpResponse, PermanentError> {
    info!(
        "Checking for available firmware updates for {}",
        params.current_version
    );

    // x-ESP32-version header

    let update_result = update_service
        .get_firmware_update(params.current_version.clone())
        .await?;

    match update_result {
        Some(update) => Ok(HttpResponse::Ok().streaming(update)),
        None => Ok(HttpResponse::NotModified().body("")),
    }
}
