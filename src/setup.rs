use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug)]
pub enum Environment {
    Local,
    Dev,
    Staging,
    Production,
}

#[derive(Debug, Parser)]
#[command(name = "TigerClaw")]
#[command(version, about="CLI wrapper to test team tiger things", long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,
    #[arg(
        short = 'n',
        long,
        help = "Environment to run against (dev, staging, production)"
    )]
    pub environmnet: String,
    #[arg(short, long, help = "Advertiser ID to test with")]
    pub advertiser_id: Option<i32>,
    #[arg(short, long, help = "External ID to test with")]
    pub external_id: Option<i32>,
    #[arg(
        short = 's',
        long = "step",
        help = "Specify the migration orchestration step to execute"
    )]
    pub migration_step: Option<String>,
    #[arg(
        short = 'c',
        long = "config",
        help = "Path to TOML configuration file specifying which tests to run"
    )]
    pub toml_config: Option<String>,
}

impl Args {
    pub fn dev() -> Self {
        Args {
            verbose: true,
            environmnet: "dev".to_string(),
            advertiser_id: Some(424242),
            external_id: Some(44911),
            migration_step: None,
            toml_config: None,
        }
    }

    pub fn get_config_path(&self) -> String {
        self.toml_config.clone().unwrap_or_else(|| {
            match self.environmnet.as_str() {
                "dev" => "tests.dev.toml",
                "staging" => "tests.staging.toml",
                "production" => "tests.prod.toml",
                _ => "tests.staging.toml",
            }
            .to_string()
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenRetrievalBody {
    access_token: String,
    expires_in: i32,
    token_type: String,
    scope: String,
}

pub async fn get_token_and_environment(args: &Args) -> (String, Environment) {
    let environment: Environment;
    let token_variable_name: String;

    match args.environmnet.as_str() {
        "local" => {
            environment = Environment::Local;
            token_variable_name = "AWIN_SPRINGFIELD_DEV_CLIENT_SECRET".to_string();
        }
        "dev" => {
            environment = Environment::Dev;
            token_variable_name = "AWIN_SPRINGFIELD_DEV_CLIENT_SECRET".to_string();
        }
        "staging" => {
            environment = Environment::Staging;
            token_variable_name = "AWIN_SPRINGFIELD_STAGING_CLIENT_SECRET".to_string();
        }
        "production" => {
            environment = Environment::Production;
            token_variable_name = "AWIN_SPRINGFIELD_PRODUCTION_CLIENT_SECRET".to_string();
        }
        _ => panic!("Invalid environment"),
    };

    let token = get_token(&token_variable_name, &environment).await;
    (token, environment)
}

async fn get_token(token_variable_name: &str, enviornmnet: &Environment) -> String {
    println!(
        "attempting to retrieve token:{} for environment: {:?}",
        token_variable_name, enviornmnet
    );
    match env::var(token_variable_name) {
        Ok(client_secret) => {
            println!(
                "Found token variable: {} client_secret:{}",
                token_variable_name, client_secret
            );
            let client = reqwest::Client::new();
            if client_secret.is_empty() {
                panic!("Client secret not found");
            }

            let params = [
                ("grant_type", "client_credentials"),
                ("client_id", "migrationTestClient"),
                ("client_secret", &client_secret),
            ];

            let url = match enviornmnet {
                Environment::Local => "http://ui.d-lhr1-docker-026.dev.awin.com/idpbackend/token",
                Environment::Dev => "http://ui.d-lhr1-docker-026.dev.awin.com/idpbackend/token",
                Environment::Staging => unimplemented!(),
                Environment::Production => unimplemented!(),
            };
            match client.post(url).form(&params).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        println!(
                            "Error: Failed to retrieve token, status code: {}, response body: {:?}",
                            response.status(),
                            response
                                .text()
                                .await
                                .unwrap_or_else(|_| "Failed to read response body".to_string())
                        );
                        return "".to_string();
                    }

                    let body = response.json::<TokenRetrievalBody>().await;
                    match body {
                        Ok(body) => {
                            let return_token = body.access_token;
                            println!(
                                "Retrieved token for environment {:?}: {}",
                                enviornmnet, return_token
                            );
                            return_token
                        }
                        Err(error) => {
                            println!("could not extract access token Error: {:?}", error);
                            "".to_string()
                        }
                    }
                }
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            }
        }
        Err(err) => {
            panic!("client secret Env variable not extracted, error: {:?}", err)
        }
    }
}
