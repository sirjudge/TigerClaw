use aws_sdk_dynamodb::{Client, Error, types::AttributeValue};
use chrono;

const PUBLISHER_TABLE_NAME: &str = "external-pub-awin-migration";

#[derive(Debug, Clone)]
pub struct Publisher {
    pub migration_name: String,
    pub external_id: String,
    pub awin_id: String,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub migration_completed: bool,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_publisher_by_external_id(external_id: i32) -> Result<Publisher, Error> {
    let config = aws_config::from_env()
        .profile_name("org-adm-springfield-dev-poweruser")
        .load()
        .await;
    let client = Client::new(&config);

    println!("Getting publisher from DynamoDB");
    let output = client
        .scan()
        .table_name(PUBLISHER_TABLE_NAME)
        .filter_expression("external_id = :external_id")
        .expression_attribute_values(":external_id", AttributeValue::S(external_id.to_string()))
        .send()
        .await?;
    println!("Output: {:?}", output);

    //TODO: Have to fix this to return the actual publisher instead of hard
    //coded values
    let publisher = Publisher {
        migration_name: "Publisher".to_string(),
        external_id: external_id.to_string(),
        awin_id: external_id.to_string(),
        end_date: None,
        migration_completed: false,
        start_date: None,
    };
    Ok(publisher)
}
