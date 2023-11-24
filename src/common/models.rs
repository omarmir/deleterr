use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_option_number_from_string};
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub host: String,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub port: Option<String>,
    pub api_key: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_ssl: bool,
    pub service: Services,
}

pub enum RequestType {
    Get,
    Delete,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseCodeBasedAction {
    pub status: APIStatus,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaExemption {
    pub request_id: usize,
    pub tmdb_id: usize,
}

impl MediaExemption {
    pub fn as_le_bytes(self) -> Vec<u8> {
        let bytes1 = self.request_id.to_le_bytes();
        let bytes2 = self.tmdb_id.to_le_bytes();
        return [bytes1, bytes2].concat();
    }
}

impl From<Vec<u8>> for MediaExemption {
    fn from(bytes: Vec<u8>) -> Self {
        let (value1, value2) = (
            usize::from_le_bytes(
                bytes[0..8]
                    .try_into()
                    .expect("Failed to read request_id from database on media exemptions"),
            ),
            usize::from_le_bytes(
                bytes[8..16]
                    .try_into()
                    .expect("Failed to read tmdb_id from database on media exemptions"),
            ),
        );
        MediaExemption {
            request_id: value1,
            tmdb_id: value2,
        }
    }
}

impl ServiceInfo {
    pub fn as_bytes(&self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self).expect("Failed to serialize to JSON");
        let bytes = json_str.as_bytes();
        bytes.to_vec()
    }
}

impl From<Vec<u8>> for ServiceInfo {
    fn from(bytes: Vec<u8>) -> Self {
        // Convert the bytes to a JSON string
        let json_str = String::from_utf8_lossy(&bytes).to_string();

        // Deserialize the JSON string into your struct
        let service_info: ServiceInfo =
            serde_json::from_str(&json_str).expect("Failed to deserialize from JSON");

        service_info
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct APIServiceStatus {
    pub service: Services,
    pub is_success: bool,
    pub status: APIStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum APIStatus {
    Success,
    WrongKey,
    Other,
    NotFound,
}

impl ToString for APIStatus {
    fn to_string(&self) -> String {
        match self {
            APIStatus::Success => "Sucess".to_string(),
            APIStatus::WrongKey => "Wrong API key provided".to_string(),
            APIStatus::Other => "Unknown error".to_string(),
            APIStatus::NotFound => "Record not found!".to_string(),
        }
    }
}

impl APIStatus {
    pub fn as_str(&self) -> &str {
        match self {
            APIStatus::Success => "Sucess",
            APIStatus::WrongKey => "Wrong API key provided",
            APIStatus::Other => "Unknown error",
            APIStatus::NotFound => "Record not found!",
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Services {
    Tautulli,
    Overseerr,
    Radarr,
    Sonarr,
}

impl ToString for Services {
    fn to_string(&self) -> String {
        match self {
            Services::Tautulli => "tautulli".to_string(),
            Services::Overseerr => "overseerr".to_string(),
            Services::Radarr => "radarr".to_string(),
            Services::Sonarr => "sonarr".to_string(),
        }
    }
}

impl Services {
    pub fn to_name(&self) -> String {
        match self {
            Services::Tautulli => "Tautulli".to_string(),
            Services::Overseerr => "Overseerr".to_string(),
            Services::Radarr => "Radarr".to_string(),
            Services::Sonarr => "Sonarr".to_string(),
        }
    }
}

pub struct RequestResponse {
    pub code: u16,
    pub status: String,
    pub response: reqwest::Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleterrError {
    details: String,
}

impl DeleterrError {
    pub fn new(msg: &str) -> DeleterrError {
        DeleterrError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DeleterrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl DeleterrError {
    pub fn add_prefix(mut self, prefix: &str) -> Self {
        self.details = format!("{} {}", prefix, self.details);
        self
    }
}

impl Error for DeleterrError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<reqwest::Error> for DeleterrError {
    fn from(err: reqwest::Error) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::IndexOpsError>> for DeleterrError {
    fn from(err: persy::PE<persy::IndexOpsError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::OpenError>> for DeleterrError {
    fn from(err: persy::PE<persy::OpenError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::BeginTransactionError>> for DeleterrError {
    fn from(err: persy::PE<persy::BeginTransactionError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::PrepareError>> for DeleterrError {
    fn from(err: persy::PE<persy::PrepareError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::GenericError>> for DeleterrError {
    fn from(err: persy::PE<persy::GenericError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::InsertError>> for DeleterrError {
    fn from(err: persy::PE<persy::InsertError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::IndexPutError>> for DeleterrError {
    fn from(err: persy::PE<persy::IndexPutError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::ReadError>> for DeleterrError {
    fn from(err: persy::PE<persy::ReadError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<persy::PE<persy::SegmentError>> for DeleterrError {
    fn from(err: persy::PE<persy::SegmentError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}
impl From<persy::PE<persy::UpdateError>> for DeleterrError {
    fn from(err: persy::PE<persy::UpdateError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}
impl From<persy::PE<persy::DeleteError>> for DeleterrError {
    fn from(err: persy::PE<persy::DeleteError>) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}
