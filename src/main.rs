use clap::Parser;
use tiger_claw::{
    config, orchestrator, sas_data_import, sas_migration_api, setup::{get_token_and_environment, Args}
};
use log::{info, warn, error};

#[tokio::main]
async fn main() {
    println!("Tiger Claw - SAS Migration Tool");
    let args = Args::parse();
    let (token, _environment) = get_token_and_environment(&args).await;
    let mut config = config::load_test_config(&args.get_config_path()).unwrap();
    if config::validate_test_config(&config).is_err() {
        error!("Configuration validation failed, exiting.");
        return;
    }
    println!("Configuration validation passed. loaded configuration from: {} and running ", args.get_config_path());

    // if we've passed in advertiser_id on top of the config file throw a warning
    if args.advertiser_id.is_some() {
        warn!("WARNING: advertiser_id passed in via command line AND the toml configuration file, this will override the config file");
        config.globals.advertiser_id = args.advertiser_id;
    }

    if args.external_id.is_some() {
        warn!("WARNING: external_id passed in via command line AND the toml configuration file, this will override the config file");
        config.globals.external_id = args.external_id;
    }

    // orchestration step should run if the step is enabled AND step_to_run is not
    // an empty string
    if config.orchestration.enabled && !config.orchestration.step_to_run.is_empty() {
        println!("Orchestration step is enabled, running...");
        orchestrator::run(
            &token,
            &config
        )
        .await;
    }
    else{
        println!("Orchestration step is not enabled or no step specified, skipping.");
    }

    // run sas_data_import step if enabled in config
    if config.sas_data_import.enabled {
        println!("SAS Data Import step is enabled, running...");
        sas_data_import::run(&config).await;
    }
    else {
        println!("SAS Data Import step is not enabled in the configuration, skipping.");
    }

    if config.migration_api.enabled {
        println!("Migration API step is enabled, running...");
        if config.globals.external_id.is_none() {
            panic!("external_id is required for migration_api");
        }
        sas_migration_api::run(
            &config.globals.external_id.unwrap(),
            &token,
            &config
        )
        .await;
    }
    else {
        println!("Migration API step is not enabled in the configuration, skipping.");
    }
}
