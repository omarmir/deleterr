use actix_web::web::Data;

use crate::common::models::DeleterrError;

use super::{
    models::{
        AppData, QueryParms, RequestStatus, RequestStatusWithRecordInfo,
        RequestStatusWithRecordInfoVector,
    },
    services::match_requests_to_watched,
};

fn update_cache(app_data: &Data<AppData>, new_data: RequestStatusWithRecordInfo) {
    let mut update_cache = app_data
        .request_cache
        .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
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

pub async fn get_requests_from_cache_or_update_cache(
    cache: Option<RequestStatusWithRecordInfo>,
    app_data: Data<AppData>,
    take: Option<usize>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
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

pub fn make_filtered_sized_vector_of_results(
    params: QueryParms,
    requests: RequestStatusWithRecordInfo,
) -> RequestStatusWithRecordInfoVector {
    let vec: Vec<RequestStatus> = requests.requests.into_values().collect();
    let final_results = match (params.take, params.skip) {
        (Some(mut take), Some(skip)) => {
            if skip > vec.len() {
                let empty_slice: &[RequestStatus; 0] = &[];
                empty_slice
            } else {
                let max_len = vec.len() - skip;
                if take > max_len {
                    take = max_len
                }
                &vec[skip..(skip + take)]
            }
        }
        (Some(mut take), None) => {
            if take > vec.len() {
                take = vec.len()
            }
            &vec[..take]
        }
        (None, Some(skip)) => {
            if skip > vec.len() {
                let empty_slice: &[RequestStatus; 0] = &[];
                empty_slice
            } else {
                &vec[skip..]
            }
        }
        _ => &vec[..],
    };

    let req_status_info = RequestStatusWithRecordInfoVector {
        all_requests: requests.all_requests,
        requests: final_results.to_vec(),
    };

    return req_status_info;
}

pub async fn get_requests_and_update_cache(
    app_data: Data<AppData>,
    params: QueryParms,
) -> Result<RequestStatusWithRecordInfoVector, DeleterrError> {
    let cache = get_cached(&app_data);
    let requests = get_requests_from_cache_or_update_cache(cache, app_data, params.take).await?;
    let req_status_info = make_filtered_sized_vector_of_results(params, requests);

    Ok(req_status_info)
}
