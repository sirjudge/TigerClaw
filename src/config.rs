use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

/// Configuration for the tests to run
///
/// This is a TOML file that specifies which tests to run.
///
/// The file should contain the following sections:
/// - [orchestration]: settings for orchestrator tests
/// - [sas_data_import]: settings for SAS data import tests
/// - [dynamo_db]: settings for DynamoDB tests
/// - [globals]: global settings like force execution
#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub orchestration: OrchestrationConfig,
    pub sas_data_import: SasDataImportConfig,
    pub dynamo_db: DynamoDbConfig,
    pub globals: GlobalConfig,
    pub migration_api: MigrationApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct MigrationApiConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct OrchestrationConfig {
    pub enabled: bool,
    pub step_to_run: String,
    pub step_status_to_force: String,
    pub force_run: bool,
}

#[derive(Debug, Deserialize)]
pub struct SasDataImportConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct DynamoDbConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct GlobalConfig {
    pub advertiser_id: Option<i32>,
    pub external_id: Option<i32>,
    pub migration_name: Option<String>,
    pub environment: String,
    pub base_growth_migration_url: String,
    pub base_growth_migration_port: i32,
    pub base_sas_data_import_url: String,
    pub base_sas_data_import_port: i32,
}

impl GlobalConfig {
    pub fn validate(&self) -> Result<(), String> {
        // Validate advertiser_id > 0 if present
        if let Some(id) = self.advertiser_id {
            if id <= 0 {
                return Err(format!(
                    "Invalid advertiser_id: {}. Must be greater than 0",
                    id
                ));
            }
        }

        // Validate external_id > 0 if present
        if let Some(id) = self.external_id {
            if id <= 0 {
                return Err(format!(
                    "Invalid external_id: {}. Must be greater than 0",
                    id
                ));
            }
        }

        // Validate migration_name is "sas" case insensitive if present
        if let Some(name) = &self.migration_name {
            if name.to_lowercase() != "sas" {
                return Err(format!(
                    "Invalid migration_name: {}. Must be 'sas' (case insensitive)",
                    name
                ));
            }
        }

        Ok(())
    }
}

pub fn load_test_config(config_path: &str) -> Result<TestConfig, String> {
    let path = Path::new(config_path);
    if !path.exists() {
        return Err(format!("Configuration file not found: {}", config_path));
    }

    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read configuration file: {}", e))?;

    toml::from_str(&contents).map_err(|e| format!("Failed to parse configuration file: {}", e))
}

pub fn validate_test_config(config: &TestConfig) -> Result<(), String> {
    if !config.orchestration.enabled
        && !config.sas_data_import.enabled
        && !config.dynamo_db.enabled
        && !config.migration_api.enabled
    {
        return Err("At least one test type must be enabled in the configuration".to_string());
    }

    // Validate global config
    config.globals.validate()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_global_config_validation() {
        // Test valid config
        let valid_config = GlobalConfig {
            advertiser_id: Some(42),
            external_id: Some(123),
            migration_name: Some("sas".to_string()),
            environment: "dev".to_string(),
            base_growth_migration_url: "http://example.com".to_string(),
            base_growth_migration_port: 8080,
            base_sas_data_import_url: "http://example.com".to_string(),
            base_sas_data_import_port: 8181,
        };
        assert!(valid_config.validate().is_ok());

        // Test valid config with uppercase migration name
        let uppercase_config = GlobalConfig {
            advertiser_id: Some(42),
            external_id: Some(123),
            migration_name: Some("SAS".to_string()),
            environment: "DEV".to_string(),
            base_growth_migration_url: "http://example.com".to_string(),
            base_growth_migration_port: 8080,
            base_sas_data_import_url: "http://example.com".to_string(),
            base_sas_data_import_port: 8181,
        };
        assert!(uppercase_config.validate().is_ok());

        // Test invalid advertiser_id
        let invalid_advertiser_id = GlobalConfig {
            advertiser_id: Some(0),
            external_id: Some(123),
            migration_name: Some("sas".to_string()),
            environment: "dev".to_string(),
            base_growth_migration_url: "http://example.com".to_string(),
            base_growth_migration_port: 8080,
            base_sas_data_import_url: "http://example.com".to_string(),
            base_sas_data_import_port: 8181,
        };
        assert!(invalid_advertiser_id.validate().is_err());

        // Test invalid external_id
        let invalid_external_id = GlobalConfig {
            advertiser_id: Some(42),
            external_id: Some(-1),
            migration_name: Some("sas".to_string()),
            environment: "dev".to_string(),
            base_growth_migration_url: "http://example.com".to_string(),
            base_growth_migration_port: 8080,
            base_sas_data_import_url: "http://example.com".to_string(),
            base_sas_data_import_port: 8181,
        };
        assert!(invalid_external_id.validate().is_err());

        // Test invalid migration_name
        let invalid_migration_name = GlobalConfig {
            advertiser_id: Some(42),
            external_id: Some(123),
            migration_name: Some("not_sas".to_string()),
            environment: "dev".to_string(),
            base_growth_migration_url: "http://example.com".to_string(),
            base_growth_migration_port: 8080,
            base_sas_data_import_url: "http://example.com".to_string(),
            base_sas_data_import_port: 8181,
        };
        assert!(invalid_migration_name.validate().is_err());
    }

    #[test]
    fn test_config_works() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "[orchestration]\nenabled = true\n\n[sas_data_import]\nenabled = false\n\n[dynamo_db]\nenabled = true").unwrap();

        let config = load_test_config(file.path().to_str().unwrap()).unwrap();
        assert!(config.orchestration.enabled);
        assert!(!config.sas_data_import.enabled);
        assert!(config.dynamo_db.enabled);
    }

    #[test]
    fn test_bad_config_fails() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not valid toml").unwrap();

        let result = load_test_config(file.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_no_tests_enabled_fails() {
        let config = TestConfig {
            orchestration: OrchestrationConfig {
                enabled: false,
                step_to_run: String::new(),
                force_run: false,
                step_status_to_force: String::new(),
            },
            sas_data_import: SasDataImportConfig { enabled: false },
            dynamo_db: DynamoDbConfig { enabled: false },
            globals: GlobalConfig {
                advertiser_id: None,
                external_id: None,
                migration_name: None,
                environment: "dev".to_string(),
                base_growth_migration_url: "http://example.com".to_string(),
                base_growth_migration_port: 8080,
                base_sas_data_import_url: "http://example.com".to_string(),
                base_sas_data_import_port: 8181,
            },
            migration_api: MigrationApiConfig { enabled: false },
        };
        assert!(validate_test_config(&config).is_err());
    }

    #[test]
    fn test_some_tests_enabled_works() {
        let config = TestConfig {
            orchestration: OrchestrationConfig {
                enabled: true,
                step_to_run: String::new(),
                force_run: false,
                step_status_to_force: String::new(),
            },
            sas_data_import: SasDataImportConfig { enabled: false },
            dynamo_db: DynamoDbConfig { enabled: false },
            globals: GlobalConfig {
                advertiser_id: None,
                external_id: None,
                migration_name: None,
                environment: "dev".to_string(),
                base_growth_migration_url: "http://example.com".to_string(),
                base_growth_migration_port: 8080,
                base_sas_data_import_url: "http://example.com".to_string(),
                base_sas_data_import_port: 8181,
            },
            migration_api: MigrationApiConfig { enabled: false },
        };
        assert!(validate_test_config(&config).is_ok());
    }

    #[test]
    fn test_parse_real_config() {
        let config = load_test_config("tests.staging.toml").unwrap();
        assert!(config.orchestration.enabled);
        assert!(config.sas_data_import.enabled);
        assert!(config.dynamo_db.enabled);
    }
}
