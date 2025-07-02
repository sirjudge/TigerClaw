use crate::{
    advertiser, config::TestConfig, orchestration_status::*, request::growth_migration_post,
};

/// Executes the orchestrator workflow for a given advertiser
///
/// # Arguments
/// * `jwt_token` - JWT token for authentication
/// * `test_config` - The test configuration
//
/// # Returns
/// This function returns nothing but may print error messages to stderr
pub async fn run(auth_token: &str, test_config: &TestConfig) {
    if test_config.globals.external_id.is_none() {
        eprintln!("No external ID provided");
        return;
    }

    let external_id = test_config.globals.external_id.unwrap();

    // Validate we have an external_id > 0
    if external_id <= 0 {
        eprintln!("Invalid external_id: {}", external_id);
        return;
    }

    // Check if the current advertiser already exists, if it doesn't then
    // initialize it
    if advertiser::get_advertiser_by_external_id(external_id)
        .await
        .is_err()
    {
        eprintln!("Advertiser with external ID {} not found or errored, re-initializing", external_id);
        match execute_step(auth_token, &external_id, "INIT", test_config).await {
            Ok(_) => {
                println!("Advertiser {} initialized successfully", external_id);
            }
            Err(error) => {
                eprintln!("Failed to initialize merchant ID {}: {}", external_id, error);
            }
        }
    }

    // if force_set_status is set, force the update to the advertiser
    if test_config.orchestration.force_run {
        println!("Force running migration for advertiser {}", external_id);
        match force_update_status(auth_token, test_config).await {
            Ok(response) => {
                println!(
                    "Successfully forced status update for merchant ID {}",
                    external_id
                );
                if response.status().is_success() {
                    println!(
                        "SAS migration started successfully for merchant_id: {}",
                        external_id
                    );
                } else {
                    eprintln!(
                        "Failed to force status update for merchant ID {}. Status: {}",
                        external_id,
                        response.status()
                    );
                }
            }
            Err(error) => {
                eprintln!(
                    "Failed to force status update for merchant ID {}: {}",
                    external_id, error
                );
                eprintln!()
            }
        }
    }
    println!(
        "Running orchestration step{} for advertiser {}",
        test_config.orchestration.step_to_run, external_id
    );

    // attempt to run the step last
    match execute_step(
        auth_token,
        &external_id,
        &test_config.orchestration.step_to_run,
        test_config,
    )
    .await
    {
        Ok(response) => {
            println!("Successfully executed step: {:?}", &test_config.orchestration.step_to_run);
            if response.status().is_success() {
                println!(
                    "SAS migration started successfully for merchant_id: {}",
                    external_id
                );
            } else {
                eprintln!(
                    "Failed to execute step for merchant ID {}. Status: {} response:{:?}",
                    external_id,
                    response.status(),
                    response.text().await
                );
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("Failed to execute step: {}", e);
        }
    }
}

/// Executes a specific step in the migration process
///
/// # Arguments
/// * `jwt_token` - JWT token for authentication
/// * `external_id` - The ID of the SAS merchant ID
/// * `step` - The step to execute
/// * `test_config` - The test configuration
///
/// # Returns
/// Returns `Ok(OrchestatorStatusReturn)` on success, or an `OrchestratorError` on failure
pub async fn execute_step(
    jwt_token: &str,
    external_id: &i32,
    step: &str,
    test_config: &TestConfig,
) -> Result<reqwest::Response, OrchestratorError> {
    let endpoint = format!(
        "migrate/sas/advertiser/{}/execute-step/{}",
        external_id, step
    );

    match growth_migration_post(&endpoint, jwt_token.to_string(), test_config).await {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => Ok(response),
            reqwest::StatusCode::NOT_FOUND => {
                let status = response
                    .json::<OrchestatorStatusReturn>()
                    .await
                    .map_err(OrchestratorError::RequestError)?;
                Err(OrchestratorError::NotFound(status))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(OrchestratorError::Unauthorized),
            status => Err(OrchestratorError::UnexpectedStatus(status)),
        },
        Err(e) => Err(OrchestratorError::RequestError(e)),
    }
}

/// Force update the migration status for an advertiser
/// # Arguments
/// * `jwt_token` - JWT token for authentication
/// * `test_config` - The test configuration
/// # Returns
/// Returns `Ok(())` on success, or an `OrchestratorError` on failure
pub async fn force_update_status(
    jwt_token: &str,
    test_config: &TestConfig,
) -> Result<reqwest::Response, OrchestratorError> {
    let external_id = test_config.globals.external_id.unwrap();
    let endpoint = format!("migrate/sas/advertiser/{}/status", external_id);

    // Prepare request body
    let request_body = serde_json::json!({
        "force_status": test_config.orchestration.step_status_to_force
    });

    // Create client
    let client = reqwest::Client::new();

    // Build the request
    let request = client
        .patch(format!(
            "{}/{}",
            test_config.globals.base_growth_migration_url, endpoint
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .json(&request_body);

    // Send the request
    match request.send().await {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK
            | reqwest::StatusCode::ACCEPTED
            | reqwest::StatusCode::NO_CONTENT => {
                println!(
                    "Successfully forced status update to '{}' for merchant ID {:?}",
                    test_config.orchestration.step_status_to_force,
                    test_config.globals.external_id
                );
                Ok(response)
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(OrchestratorError::Unauthorized),
            reqwest::StatusCode::NOT_FOUND => {
                let status = response
                    .json::<OrchestatorStatusReturn>()
                    .await
                    .map_err(OrchestratorError::RequestError)?;
                Err(OrchestratorError::NotFound(status))
            }
            status => {
                println!("Unexpected status code: {}", status);
                Err(OrchestratorError::UnexpectedStatus(status))
            }
        },
        Err(e) => Err(OrchestratorError::RequestError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_test_config;
    use crate::setup::{self};

    /// Helper function to get the environment and token for tests
    async fn get_environment_and_token() -> (String, setup::Environment) {
        let args = setup::Args {
            verbose: true,
            environmnet: "dev".to_string(),
            advertiser_id: Some(424242),
            external_id: Some(242424),
            migration_step: None,
            toml_config: None,
        };

        setup::get_token_and_environment(&args).await
    }

    #[tokio::test]
    async fn force_update_advertiser() {
        let (token, _environment) = get_environment_and_token().await;
        let mut config = load_test_config("tests.local.toml").unwrap();
        config.orchestration.step_status_to_force = "COMPLETED".to_string();
        super::force_update_status(&token, &config).await.unwrap();
    }

    /// This test will iterate through each step in the StepDescriptor enum
    /// and execute it against a real API, so it should be marked as #[ignore]
    /// to avoid running it during normal test runs.
    #[tokio::test]
    #[ignore]
    async fn test_execute_all_steps() {
        let (token, _environment) = get_environment_and_token().await;

        // Set up test data
        let advertiser_id = 424242;

        // Define all the steps we want to test
        let steps = vec![
            "INIT", "VALID", "SF", "ADV", "PUB", "TRACK", "VOUCH", "MEM_TAG", "COM", "FEE", "FEED",
            "CREATIVE",
        ];

        let config = load_test_config("tests.local.toml").unwrap();

        for step in steps {
            println!("Testing step: {}", step);

            // Execute the step
            match execute_step(&token, &advertiser_id, step, &config).await {
                Ok(response) => {
                    let status = response.status();
                    println!("  Success! Status: {:?}", status);
                }
                Err(e) => {
                    // We don't assert here because some steps might legitimately fail
                    // in a real environment, especially if prerequisites aren't met
                    println!("  Error executing step {}: {}", step, e);
                }
            }

            // Sleep briefly between API calls to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    /// Test the force_update_status function
    #[tokio::test]
    #[ignore]
    async fn test_force_update_status() {
        let (token, _environment) = get_environment_and_token().await;

        // Set up test data
        // Test the force update function with various statuses
        let statuses = vec!["INIT_DONE", "VALID_DONE", "COMPLETED"];

        // load local toml config
        let mut config = load_test_config("tests.local.toml").unwrap();

        for status in statuses {
            println!("Testing force update to status: {}", status);
            config.orchestration.step_status_to_force = status.to_string();

            match force_update_status(&token, &config).await {
                Ok(_) => {
                    println!("  Successfully forced status to {}", status);
                }
                Err(e) => {
                    println!("  Error forcing status to {}: {}", status, e);
                    // We don't fail the test as some statuses might not be valid in certain contexts
                }
            }

            // Sleep briefly between API calls
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}
