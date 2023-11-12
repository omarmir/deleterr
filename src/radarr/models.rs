use serde::Serialize;

use crate::common::models::APIStatus;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrDeleteResponse {
    pub status: APIStatus,
    pub is_success: bool,
}
