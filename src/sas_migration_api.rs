use crate::config::TestConfig;
use crate::request::growth_migration_post;

pub async fn run(merchant_id: &i32, token: &str, test_config: &TestConfig) {
    // Validate that the merchant_id is greater than 0
    if *merchant_id <= 0 {
        let error_message = format!(
            "Invalid merchant_id: {}. Must be greater than 0",
            merchant_id
        );
        panic!("{}", error_message);
    }

    enable_lockdown(merchant_id, token, test_config).await;
    enable_fee_lock(merchant_id, token, test_config).await;
}

async fn enable_lockdown(merchant_id: &i32, token: &str, test_config: &TestConfig) {
    println!("Enabling lockdown...");
    let endpoint_path = format!("sasMigrationApi/lockdown/{}", merchant_id);
    match growth_migration_post(&endpoint_path, token.to_string(), test_config).await {
        Ok(response) => {
            println!("Response: {:?}", response);
            if response.status().is_success() {
                println!(
                    "SAS migration started successfully for merchant_id: {}",
                    merchant_id
                );
            } else {
                let error_message = format!(
                    "Failed to lock merchant_id: {}. Status: {}",
                    merchant_id,
                    response.status()
                );
                eprintln!("{}", error_message);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            let error_message = format!(
                "Failed to lock merchant_id: {}. Error Message: {}",
                merchant_id, e
            );
            eprintln!("{}", error_message);
        }
    };
}

async fn enable_fee_lock(merchant_id: &i32, token: &str, test_config: &TestConfig) {
    println!("Enabling fee lock...");
    let endpoint_path = format!("sasMigrationApi/feelock/{}", merchant_id);
    match growth_migration_post(&endpoint_path, token.to_string(), test_config).await {
        Ok(response) => {
            println!("Response: {:?}", response);
            if response.status().is_success() {
                println!(
                    "SAS migration started successfully for merchant_id: {}",
                    merchant_id
                );
            } else {
                let error_message = format!(
                    "response Failed to fee lock merchant_id: {}. Status: {}",
                    merchant_id,
                    response.status()
                );
                eprintln!("{}", error_message);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            let error_message = format!(
                "Failed to fee lock merchant_id: {}. Error Message: {}",
                merchant_id, e
            );
            eprintln!("{}", error_message);
        }
    };
}
