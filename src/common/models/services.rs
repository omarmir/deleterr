use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_option_number_from_string};

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

impl ServiceInfo {
    pub fn as_vec(&self) -> Vec<u8> {
        let service_info_bytes = serde_json::to_vec(&self).expect("Failed to serialize to JSON");
        service_info_bytes
    }
}

impl From<&[u8]> for ServiceInfo {
    fn from(bytes: &[u8]) -> Self {
        let service_info: ServiceInfo =
            serde_json::from_slice(bytes).expect("Failed to deserialize service info");

        service_info
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
