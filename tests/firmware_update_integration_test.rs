mod agent;

#[cfg(feature = "integration")]
#[cfg(test)]
mod tests {
    use crate::agent::{build_coriolis_api_agent, CoriolisApiRequest};
    use serde_json::from_str;

    #[tokio::test]
    async fn test_firmware_update() {
        let agent = build_coriolis_api_agent().await.unwrap();

        let result = agent
            .call(CoriolisApiRequest {
                uri: "/operational/firmware_update".to_string(),
                http_method: "GET".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(result.status, 200);
    }
}
