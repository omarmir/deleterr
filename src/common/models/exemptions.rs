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
