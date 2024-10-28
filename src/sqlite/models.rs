use crate::overseerr::models::RequestSeason;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct RequestsWithStatus {
    pub id: usize,
    pub request_status: RequestSeason,
}
