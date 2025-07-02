use crate::config::TestConfig;
use crate::merchant::Merchant;
use log::{error, warn};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct SasDataImport {
    pub status: String,
}

pub async fn run(test_config: &TestConfig) {
    let client = reqwest::Client::new();
    health_check(client.clone(), test_config).await;
    let api_key = get_api_key().expect("Error: sas_data_import API key not found");

    if test_config.globals.external_id.is_none() {
        error!("No external_id provided",);
        return;
    } else if test_config.globals.external_id.unwrap() <= 0 {
        error!(
            "Invalid external_id: {}. Must be greater than 0",
            test_config.globals.external_id.unwrap()
        );
        return;
    }

    let merchant = merchant_extraction(client.clone(), test_config, &api_key).await;
    println!("Merchant extraction completed: {:?}", merchant);
}

fn get_api_key() -> Option<String> {
    match env::var("AWIN_SAS_DATA_IMPORT_API_SECRET") {
        Ok(secret) => {
            println!("Found SAS Data Import API secret");
            Some(secret)
        }
        Err(_) => {
            error!("SAS Data Import API secret not found in environment variables");
            None
        }
    }
}


/// Runs a ping to the SAS Data Import service to check if it is running on the given URL set above
async fn health_check(client: reqwest::Client, test_config: &TestConfig) {
    let health_check_url = format!(
        "{}/actuator/health",
        test_config.globals.base_sas_data_import_url
    );
    println!("running health check on: {}", health_check_url);
    match client.get(health_check_url).send().await {
        Ok(response) => {
            println!(
                "Received response from SAS Data Import service: {:?}",
                response
            );
            match response.json::<SasDataImport>().await {
                Ok(status) => {
                    if status.status != "UP" {
                        println!(
                            "ERROR:SAS Data Import service is not returning UP status: {:?}",
                            status
                        );
                    }
                    println!(
                        "SAS Data Import service is running with status: {:?}",
                        status.status
                    );
                }
                Err(error) => {
                    println!(
                        "ERROR:Failed to parse response from SAS data import: {:?}",
                        error
                    );
                }
            }
        }
        Err(error) => {
            println!("ERROR:sas_data_import ping request error: {:?}", error);
        }
    }

    println!("Health check method completed");
}

async fn merchant_extraction(
    client: reqwest::Client,
    test_config: &TestConfig,
    api_key: &str,
) -> Result<Merchant, Box<dyn std::error::Error>> {
    println!("Running merchant extraction...");
    let url = format!(
        "{}:{}/merchant/{}",
        test_config.globals.base_sas_data_import_url,
        test_config.globals.base_sas_data_import_port,
        test_config.globals.external_id.unwrap()
    );

    println!("Extracting merchant data using URL: {}", url);
    let response = client
        .get(url)
        .header(AUTHORIZATION, api_key)
        .send()
        .await?;

    if response.status() != 200 {
        warn!("Failed to extract merchant data: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to extract merchant data: {:?}", response.status()),
        )));
    }

    match response.json::<Merchant>().await {
        Ok(merchant) => {
            println!("Merchant data extracted successfully: {:?}", merchant);
            Ok(merchant)
        }
        Err(e) => {
            error!("Failed to parse merchant data: {:?}", e);
            return Err(Box::new(e));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{self, load_test_config};

    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let client = reqwest::Client::new();
        let test_config = config::load_test_config("tests.local.toml").unwrap();
        health_check(client, &test_config).await;
    }

    #[tokio::test]
    async fn test_merchant_extraction() {
        let client = reqwest::Client::new();
        let mut test_config = config::load_test_config("tests.local.toml").unwrap();
        let api_key = get_api_key().expect("API key not found");
        test_config.globals.external_id = Some(44911);
        match merchant_extraction(client, &test_config, &api_key).await {
            Ok(merchant) => {
                assert!(
                    merchant.merchant_id == 44911,
                    "Merchant extraction should succeed"
                );
            }
            Err(e) => {
                panic!("Merchant extraction failed: {:?}", e);
            }
        }
    }
}
