use crate::config::TestConfig;
use reqwest::Response;

pub async fn growth_migration_get(
    endpoint_path: &str,
    jwt_token: &str,
    test_config: &TestConfig,
) -> Result<Response, reqwest::Error> {
    let url = format!(
        "{}:{}/{}",
        test_config.globals.base_growth_migration_url,
        test_config.globals.base_growth_migration_port,
        endpoint_path
    );
    let client = reqwest::Client::new();
    client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
}

pub async fn growth_migration_post(
    endpoint_path: &str,
    jwt_token: String,
    test_config: &TestConfig,
) -> Result<Response, reqwest::Error> {
    let url = format!(
        "{}:{}/{}",
        test_config.globals.base_growth_migration_url,
        test_config.globals.base_growth_migration_port,
        endpoint_path
    );
    let client = reqwest::Client::new();
    client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
}

pub async fn sas_data_import_get(
    endpoint_path: &str,
    jwt_token: &str,
    config: &TestConfig,
) -> Result<Response, reqwest::Error> {
    let url = format!(
        "{}/{}",
        config.globals.base_sas_data_import_url, endpoint_path
    );
    let client = reqwest::Client::new();
    client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
}

pub async fn sas_data_import_put(
    endpoint_path: &str,
    jwt_token: String,
    test_config: &TestConfig,
) -> Response {
    let url = format!(
        "{}/{}",
        test_config.globals.base_sas_data_import_url, endpoint_path
    );
    let client = reqwest::Client::new();
    match client
        .put(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            panic!("put request error: {:?}", error);
        }
    }
}
