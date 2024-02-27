use serde::{Deserialize, Serialize};

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

impl MediaExemption {
    pub fn as_vec(&self) -> Vec<u8> {
        let media_exemption_bytes =
            serde_json::to_vec(&self).expect("Failed to serialize media exemption to JSON");
        media_exemption_bytes
    }
}

impl From<&[u8]> for MediaExemption {
    fn from(bytes: &[u8]) -> Self {
        let media_exemption: MediaExemption =
            serde_json::from_slice(bytes).expect("Failed to deserialize media exemption");

        media_exemption
    }
}
