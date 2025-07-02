use crate::request::growth_migration_get;
use crate::config::TestConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Terms {
    pub term_status: String,
    pub term_params: TermParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermParams {
    pub external_program_id: i32,
    pub external_program_name: String,
    pub awin_tech_fee: i32,
    pub tech_bundle: i32,
    pub tracking_fee_type: String,
    pub service_package: String,
    pub tracking_fee: i32,
    pub validation_period: i32,
}

impl TermParams {
    pub fn validate(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if self.external_program_id <= 0 {
            errors.push("External program ID must be greater than 0".to_string());
        }
        if self.external_program_name.is_empty() {
            errors.push("External program name must not be empty".to_string());
        }
        if self.awin_tech_fee < 0 {
            errors.push("AWIN tech fee must be non-negative".to_string());
        }
        errors
    }
}

pub async fn run(jwt_token: &str, advertiser_id: &i32, test_config: &TestConfig) {
    get_advertiser_terms_status(jwt_token, advertiser_id, test_config).await;
    println!("completed terms and conditions test, no failures");
}

async fn get_advertiser_terms_status(jwt_token: &str, advertiser_id: &i32, test_config: &TestConfig) {
    let terms_endpoint = format!("terms/sas/advertiser/awin/{}", advertiser_id);
    let response = growth_migration_get(&terms_endpoint, jwt_token, test_config)
        .await
        .expect("Failed to get endpoint");

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Terms>().await {
            Ok(terms) => {
                assert!(terms.term_params.external_program_id > 0);
                assert!(!terms.term_params.external_program_name.is_empty());
                println!("Advertiser terms: {:?}", terms);
            }
            Err(error) => {
                panic!("Failed to parse response: {:?}", error);
            }
        },
        reqwest::StatusCode::NOT_FOUND => {
            panic!("Advertiser not found or terms not available")
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized access: Invalid JWT token");
        }
        _ => {
            panic!("Unexpected status code: {:?}", response.status());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn term_validation_successful() {
        let valid_terms = Terms {
            term_status: "active".to_string(),
            term_params: TermParams {
                external_program_id: 1,
                external_program_name: "Test Program".to_string(),
                awin_tech_fee: 100,
                tech_bundle: 1,
                tracking_fee_type: "fixed".to_string(),
                // let advertiser_id = "31858";
                service_package: "basic".to_string(),
                tracking_fee: 50,
                validation_period: 30,
            },
        };

        let errors = valid_terms.term_params.validate();
        assert!(errors.is_empty(), "Validation errors: {:?}", errors);
    }

    #[tokio::test]
    async fn empty_program_name() {
        let valid_terms = Terms {
            term_status: "active".to_string(),
            term_params: TermParams {
                external_program_id: 1,
                external_program_name: "".to_string(),
                awin_tech_fee: 100,
                tech_bundle: 1,
                tracking_fee_type: "test_type".to_string(),
                service_package: "super cool guy tier".to_string(),
                tracking_fee: 50,
                validation_period: 30,
            },
        };

        let errors = valid_terms.term_params.validate();
        assert!(
            errors.len() == 1,
            "Expecte exactly 1 validation error but saw {}",
            errors.len()
        );
    }
}
