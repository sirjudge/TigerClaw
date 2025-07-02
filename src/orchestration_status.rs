use serde::{Deserialize, Serialize};
use std::fmt;

impl std::error::Error for OrchestratorError {}
/// Represents the status response from the orchestrator API
#[derive(Debug, Serialize, Deserialize)]
pub struct OrchestatorStatusReturn {
    pub timestamp: String,
    pub status: i16,
    pub error: String,
    pub path: String,
}

/// Possible errors that can occur during orchestrator operations
#[derive(Debug)]
pub enum OrchestratorError {
    InvalidAdvertiserId,
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    NotFound(OrchestatorStatusReturn),
    Unauthorized,
    UnexpectedStatus(reqwest::StatusCode),
}

impl fmt::Display for OrchestratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrchestratorError::InvalidAdvertiserId => {
                write!(f, "Advertiser ID must be greater than 0")
            }
            OrchestratorError::RequestError(e) => write!(f, "Request error: {}", e),
            OrchestratorError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            OrchestratorError::NotFound(status) => write!(f, "Advertiser not found: {:?}", status),
            OrchestratorError::Unauthorized => write!(f, "Unauthorized access: Invalid JWT token"),
            OrchestratorError::UnexpectedStatus(status) => {
                write!(f, "Unexpected status code: {}", status)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    // Initialization states
    InitDone(StepDescriptor),
    InitErr(StepDescriptor),
    InitRun(StepDescriptor),

    // Validation states
    ValidRun(StepDescriptor),
    ValidDone(StepDescriptor),
    ValidErr(StepDescriptor),

    // Salesforce states
    SfRun(StepDescriptor),
    SfDone(StepDescriptor),
    SfErr(StepDescriptor),

    // Advertiser states
    AdvRun(StepDescriptor),
    AdvDone(StepDescriptor),
    AdvErr(StepDescriptor),

    // Publisher states
    PubRun(StepDescriptor),
    PubDone(StepDescriptor),
    PubErr(StepDescriptor),

    // Tracking states
    TrackRun(StepDescriptor),
    TrackDone(StepDescriptor),
    TrackErr(StepDescriptor),

    // Voucher states
    VouchRun(StepDescriptor),
    VouchDone(StepDescriptor),
    VouchErr(StepDescriptor),

    // Membership tag states
    MemTagRun(StepDescriptor),
    MemTagDone(StepDescriptor),
    MemTagErr(StepDescriptor),

    // Commission states
    ComRun(StepDescriptor),
    ComDone(StepDescriptor),
    ComErr(StepDescriptor),

    // Fee states
    FeeRun(StepDescriptor),
    FeeDone(StepDescriptor),
    FeeErr(StepDescriptor),

    // Feed states
    FeedRun(StepDescriptor),
    FeedDone(StepDescriptor),
    FeedErr(StepDescriptor),

    // Creative states
    CreativeRun(StepDescriptor),
    CreativeDone(StepDescriptor),
    CreativeErr(StepDescriptor),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepDescriptor {
    Init,
    Valid,
    Sf,
    Adv,
    Pub,
    Track,
    Vouch,
    MemTag,
    Com,
    Fee,
    Feed,
    Creative,
}

impl StepDescriptor {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INIT" => Some(StepDescriptor::Init),
            "VALID" => Some(StepDescriptor::Valid),
            "SF" => Some(StepDescriptor::Sf),
            "ADV" => Some(StepDescriptor::Adv),
            "PUB" => Some(StepDescriptor::Pub),
            "TRACK" => Some(StepDescriptor::Track),
            "VOUCH" => Some(StepDescriptor::Vouch),
            "MEM_TAG" => Some(StepDescriptor::MemTag),
            "COM" => Some(StepDescriptor::Com),
            "FEE" => Some(StepDescriptor::Fee),
            "FEED" => Some(StepDescriptor::Feed),
            "CREATIVE" => Some(StepDescriptor::Creative),
            _ => None,
        }
    }
}

impl MigrationStatus {
    pub fn from_string(s: &str, step: StepDescriptor) -> Option<Self> {
        match s.to_uppercase().as_str() {
            // Initialization states
            "INIT_DONE" => Some(MigrationStatus::InitDone(step)),
            "INIT_ERR" => Some(MigrationStatus::InitErr(step)),
            "INIT_RUN" => Some(MigrationStatus::InitRun(step)),

            // Validation states
            "VALID_RUN" => Some(MigrationStatus::ValidRun(step)),
            "VALID_DONE" => Some(MigrationStatus::ValidDone(step)),
            "VALID_ERR" => Some(MigrationStatus::ValidErr(step)),

            // Salesforce states
            "SF_RUN" => Some(MigrationStatus::SfRun(step)),
            "SF_DONE" => Some(MigrationStatus::SfDone(step)),
            "SF_ERR" => Some(MigrationStatus::SfErr(step)),

            // Advertiser states
            "ADV_RUN" => Some(MigrationStatus::AdvRun(step)),
            "ADV_DONE" => Some(MigrationStatus::AdvDone(step)),
            "ADV_ERR" => Some(MigrationStatus::AdvErr(step)),

            // Publisher states
            "PUB_RUN" => Some(MigrationStatus::PubRun(step)),
            "PUB_DONE" => Some(MigrationStatus::PubDone(step)),
            "PUB_ERR" => Some(MigrationStatus::PubErr(step)),

            // Tracking states
            "TRACK_RUN" => Some(MigrationStatus::TrackRun(step)),
            "TRACK_DONE" => Some(MigrationStatus::TrackDone(step)),
            "TRACK_ERR" => Some(MigrationStatus::TrackErr(step)),

            // Voucher states
            "VOUCH_RUN" => Some(MigrationStatus::VouchRun(step)),
            "VOUCH_DONE" => Some(MigrationStatus::VouchDone(step)),
            "VOUCH_ERR" => Some(MigrationStatus::VouchErr(step)),

            // Membership tag states
            "MEM_TAG_RUN" => Some(MigrationStatus::MemTagRun(step)),
            "MEM_TAG_DONE" => Some(MigrationStatus::MemTagDone(step)),
            "MEM_TAG_ERR" => Some(MigrationStatus::MemTagErr(step)),

            // Commission states
            "COM_RUN" => Some(MigrationStatus::ComRun(step)),
            "COM_DONE" => Some(MigrationStatus::ComDone(step)),
            "COM_ERR" => Some(MigrationStatus::ComErr(step)),

            // Fee states
            "FEE_RUN" => Some(MigrationStatus::FeeRun(step)),
            "FEE_DONE" => Some(MigrationStatus::FeeDone(step)),
            "FEE_ERR" => Some(MigrationStatus::FeeErr(step)),

            // Feed states
            "FEED_RUN" => Some(MigrationStatus::FeedRun(step)),
            "FEED_DONE" => Some(MigrationStatus::FeedDone(step)),
            "FEED_ERR" => Some(MigrationStatus::FeedErr(step)),

            // Creative states
            "CREATIVE_RUN" => Some(MigrationStatus::CreativeRun(step)),
            "CREATIVE_DONE" => Some(MigrationStatus::CreativeDone(step)),
            "CREATIVE_ERR" => Some(MigrationStatus::CreativeErr(step)),

            _ => None,
        }
    }
}
