use serde::{Deserialize, Serialize};

/*
example response from the API
Response text: Ok("{\"merchantId\":44911,\"organization\":\"TraditionalFamilyRecipes\",\"username\":\"Traditionalfamilyrecipes1\",\"email\":\"alvin.ng@shareasale.com\",\"firstName\":\"ShareASale\",\"lastName\":\"44911-NoLName\",\"address\":\"43 Bear Mountain Rd Ste 501\",\"address2\":\"\",\"city\":\"Ringwood\",\"state\":\"NJ\",\"country\":\"US\",\"zip\":\"07456-2901\",\"phone\":\"7735696750\",\"bio\":\"\",\"agreement\":\"Advertising Policy:\\r\\n\\r\\nAffiliates may NOT use branded terms, such as \\\"TF\\\", \\\"TF Jewelry\\\", or \\\"work for TFs\\\" in paid (PPC) advertising. This includes both ad copy and targeted keywords\\r\\n\\r\\nAffiliates must clearly not represent themselves as ITF corporate office. \\r\\n\\r\\nAmbassador Policy: \\r\\n\\r\\nAt this time, TF Ambassadors cannot be affiliates.  \\r\\n\\r\\nTest Test 7/7/2021\",\"category\":\"28,25,53,2,3\",\"logoFile\":\"/image/44911/marketing/211175_0.gif\",\"dataFeeds\":0,\"externalId\":31886,\"isPrivate\":false,\"advertiserPlatformPlan\":\"Advanced\",\"approved\":null,\"stepupstep\":null,\"balance\":null,\"creditLimit\":null}")
Merchant extraction completed: Ok(Merchant { merchant_id: 44911, organization: "Test Organization", username: "testuser", email: "nicojudge@nico.com", first_name: "Nico", last_name: "Judge", address: "123 Test Street", address2: "Apt 4B", city: "Test City", state: "TS", country: "Test Country", zip: "12345", phone: "+1-555-555-5555", bio: "Test merchant biography", category: "Test Category", agreement: "Test Agreement", logo_file: "test_logo.png", data_feeds: Some(42), external_id: 98765, is_private: Some(false), approved: Some(true), setup_step: "completed", balance: Some(1000.5), credit_limit: Some(5000.0), advertiser_platform_plan: "premium" })

 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Merchant {
    #[serde(rename = "merchantId")]
    pub merchant_id: i64,
    pub organization: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub address: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
    pub phone: String,
    pub bio: String,
    pub category: String,
    pub agreement: String,
    #[serde(rename = "logoFile")]
    pub logo_file: String,
    #[serde(rename = "dataFeeds")]
    pub data_feeds: Option<i64>,
    #[serde(rename = "externalId")]
    pub external_id: i64,
    #[serde(rename = "isPrivate")]
    pub is_private: Option<bool>,
    pub approved: Option<bool>,
    #[serde(rename = "stepupstep")]
    pub setup_step: Option<String>,
    pub balance: Option<f64>,
    #[serde(rename = "creditLimit")]
    pub credit_limit: Option<f64>,
    #[serde(rename = "advertiserPlatformPlan")]
    pub advertiser_platform_plan: String,
}

impl Merchant {
    pub fn test_merchant() -> Self {
        Merchant {
            merchant_id: 44911,
            organization: "Test Organization".to_string(),
            username: "testuser".to_string(),
            email: "nicojudge@nico.com".to_string(),
            first_name: "Nico".to_string(),
            last_name: "Judge".to_string(),
            address: "123 Test Street".to_string(),
            address2: "Apt 4B".to_string(),
            city: "Test City".to_string(),
            state: "TS".to_string(),
            country: "Test Country".to_string(),
            zip: "12345".to_string(),
            phone: "+1-555-555-5555".to_string(),
            bio: "Test merchant biography".to_string(),
            category: "Test Category".to_string(),
            agreement: "Test Agreement".to_string(),
            logo_file: "test_logo.png".to_string(),
            data_feeds: Some(42),
            external_id: 98765,
            is_private: Some(false),
            approved: Some(true),
            setup_step: Some("completed".to_string()),
            balance: Some(1000.50),
            credit_limit: Some(5000.00),
            advertiser_platform_plan: "premium".to_string(),
        }
    }
}
