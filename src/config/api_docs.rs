use crate::controller::operational_controller as operational;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(operational::firmware_update))]
pub struct ApiDoc;
