use actix_web::web::Data;

use crate::common::models::DeleterrError;

use super::{
    models::{AppData, RequestStatusWithRecordInfo},
    services::match_requests_to_watched,
};

fn update_cache(app_data: &Data<AppData>, new_data: RequestStatusWithRecordInfo) {
    let mut update_cache = app_data
        .request_cache
        .write()
        .expect("Unable to access cache");

    *update_cache = Some(new_data);
}

fn get_cached(app_data: &Data<AppData>) -> Option<RequestStatusWithRecordInfo> {
    let cache = app_data
        .request_cache
        .read()
        .expect("Unable to access cache");

    let unlocked_cache = &*cache;

    let resp = match unlocked_cache {
        Some(requests) => Some(requests.clone()),
        None => None,
    };

    resp
}

pub async fn get_requests_and_update_cache(
    app_data: Data<AppData>,
    take: Option<usize>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let cache = get_cached(&app_data);
    let req = match cache {
        Some(cached) => cached,
        None => {
            let request_status_with_record_info = match_requests_to_watched(take).await?;
            update_cache(&app_data, request_status_with_record_info.clone());
            request_status_with_record_info
        }
    };

    Ok(req)
}
