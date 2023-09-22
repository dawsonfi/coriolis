use actix_web::web::Data;
use coriolis::config::api_docs::ApiDoc;
use coriolis::config::telemetry::{get_subscriber, init_subscriber};
use coriolis::controller::operational_controller::firmware_update;
use coriolis::repository::ConfigProvider;
use coriolis::service::UpdateService;
use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let subscriber = get_subscriber("coriolis-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config_provider = ConfigProvider::default().provide().await;
    let factory = move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(firmware_update)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(Data::new(UpdateService::new(&config_provider)))
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind(("127.0.0.1", 8080))?
            .run()
            .await?;
    }
    Ok(())
}
