use crate::orchestration_status::{MigrationStatus, StepDescriptor};
use aws_sdk_dynamodb::{
    Client, Error, types::AttributeValue, types::error::ResourceNotFoundException,
};
const ADVERTISER_TABLE_NAME: &str = "external-adv-awin-migration";

#[derive(Debug, Clone)]
pub struct Advertiser {
    pub migration_name: String,
    pub awin_id: String,
    pub external_id: String,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub migration_completed: bool,
    pub migration_status_string: String,
    pub migration_status: MigrationStatus,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub terms_awin_user_id: String,
    pub terms_status: String,
    pub terms_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl Advertiser {
    pub fn validate(&self) -> Result<(), String> {
        let mut validation_errors: Vec<String> = Vec::new();
        // we only have "sas" as a migration name for now, anything else
        // and we can consider it invalid
        if self.migration_name != "sas" {
            validation_errors.push("Migration name must be sas".to_string());
        }
        // make sure we have both awin and external (SAS) IDs
        if self.awin_id.is_empty() {
            validation_errors.push("Awin ID is required".to_string());
        }
        if self.external_id.is_empty() {
            validation_errors.push("External ID is required".to_string());
        }


        // if we have no validation errors, return Ok() else return error()
        if validation_errors.is_empty() {
            return Ok(());
        }

        Err(validation_errors.join(", "))
    }
}

/// Get advertiser by external ID, note: external_id in this case is just the SAS Merchant ID
pub async fn get_advertiser_by_external_id(external_id: i32) -> Result<Advertiser, Error> {
    let config = aws_config::from_env()
        .profile_name("org-adm-springfield-dev-poweruser")
        .load()
        .await;
    let client = Client::new(&config);

    let output = client
        .scan()
        .table_name(ADVERTISER_TABLE_NAME)
        .filter_expression("external_id = :external_id")
        .expression_attribute_values(":external_id", AttributeValue::S(external_id.to_string()))
        .send()
        .await?;

    let items = output.items.unwrap_or_default();
    //TODO: Handle this better methinks
    if items.is_empty() {
        return Err(Error::ResourceNotFoundException(
            ResourceNotFoundException::builder()
                .message(format!("No advertiser found for id: {}", external_id))
                .build(),
        ));
    }

    let item = &items[0];
    let advertiser = Advertiser {
        migration_name: item
            .get("migration_name")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&String::from("sas"))
            .to_string(),
        awin_id: item
            .get("awin_id")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&external_id.to_string())
            .to_string(),
        external_id: item
            .get("external_id")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&external_id.to_string())
            .to_string(),
        end_date: item
            .get("end_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        migration_completed: item
            .get("migration_completed")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false)
            .to_owned(),
        migration_status_string: item
            .get("migration_status")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&String::from("Pending"))
            .to_string(),
        migration_status: MigrationStatus::from_string(
            &item
                .get("migration_status")
                .and_then(|v| v.as_s().ok())
                .unwrap_or(&String::from("Pending"))
                .to_string(),
            StepDescriptor::Init,
        )
        .unwrap_or(MigrationStatus::InitRun(StepDescriptor::Init)),
        start_date: item
            .get("start_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        terms_awin_user_id: item
            .get("terms_awin_user_id")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&external_id.to_string())
            .to_string(),
        terms_status: item
            .get("terms_status")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&String::from("Pending"))
            .to_string(),
        terms_timestamp: item
            .get("terms_timestamp")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
    };
    Ok(advertiser)
}

// NOTE: This function is not currently used as I want to make sure
// I don't accidentally delete important data
pub async fn delete_advertiser_by_external_id(external_id: i32) -> Result<(), Error> {
    let config = aws_config::from_env()
        .profile_name("org-adm-springfield-dev-poweruser")
        .load()
        .await;
    let client = Client::new(&config);

    client
        .delete_item()
        .table_name(ADVERTISER_TABLE_NAME)
        .key("external_id", AttributeValue::S(external_id.to_string()))
        .send()
        .await?;

    Ok(())
}

// NOTE: This function is not currently used as I want to make sure
// I don't accidentally delete important data
pub async fn delete_advertiser_by_awin_advertiser_id(awin_id: i32) -> Result<(), Error> {
    let config = aws_config::from_env()
        .profile_name("org-adm-springfield-dev-poweruser")
        .load()
        .await;
    let client = Client::new(&config);

    client
        .delete_item()
        .table_name(ADVERTISER_TABLE_NAME)
        .key("awin_id", AttributeValue::S(awin_id.to_string()))
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_advertiser() {
        let external_id = 424242;
        let advertiser = get_advertiser_by_external_id(external_id).await;
        match advertiser {
            Ok(advertiser) => {
                println!("Advertiser: {:?}", advertiser);
                assert_eq!(advertiser.external_id, "424242");
                assert_eq!(advertiser.migration_name, "sas");
                // assert_eq!(advertiser.migration_status, "Pending");
                // assert_eq!(advertiser.start_date, None);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
