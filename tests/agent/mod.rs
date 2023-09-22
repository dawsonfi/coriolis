mod coriolis_api_lambda_agent;

use async_trait::async_trait;
#[cfg(feature = "integration")]
use coriolis_api_lambda_agent::CoriolisApiLambdaAgent;
use std::collections::HashMap;
#[cfg(feature = "integration")]
use std::env;
use std::error::Error;

#[derive(Default)]
pub struct CoriolisApiRequest {
    pub uri: String,
    pub http_method: String,
    pub payload: Option<String>,
    pub query_parameters: Option<HashMap<String, String>>,
}

pub struct CoriolisApiResponse {
    pub status: i32,
    pub payload: String,
}

#[async_trait]
pub trait CoriolisApiAgent {
    async fn call(
        &self,
        request: CoriolisApiRequest,
    ) -> Result<CoriolisApiResponse, Box<dyn Error>>;
}

#[cfg(feature = "integration")]
pub async fn build_coriolis_api_agent() -> Result<Box<dyn CoriolisApiAgent>, Box<dyn Error>> {
    Ok(match env::var("RUNNING_ENV") {
        Ok(running_env) => Box::new(CoriolisApiLambdaAgent::new(running_env).await),
        Err(_) => Box::new(CoriolisApiLambdaAgent::new("beta".to_string()).await), //Defaults to beta
    })
}
