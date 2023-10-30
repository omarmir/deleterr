use crate::{
    common::models::DeleterrError, overseerr::models::MediaType, tautulli::models::UserWatchHistory,
};
use actix_web::web::Data;
use std::cmp::Ordering;

use super::{
    models::{
        AppData, QueryParms, RequestStatus, RequestStatusWithRecordInfo,
        RequestStatusWithRecordInfoVector, SortableProps,
    },
    services::match_requests_to_watched,
};

fn compare_media_variants(a: &MediaType, b: &MediaType) -> Ordering {
    match (a, b) {
        (MediaType::Movie, MediaType::Movie) => Ordering::Equal,
        (MediaType::TV, MediaType::TV) => Ordering::Equal,
        (MediaType::Movie, MediaType::TV) => Ordering::Less,
        (MediaType::TV, MediaType::Movie) => Ordering::Greater,
    }
}

fn compare_user_watch_history(
    a: &Option<UserWatchHistory>,
    b: &Option<UserWatchHistory>,
) -> Ordering {
    match (a, b) {
        (Some(_), None) => Ordering::Less,    // None < Some
        (None, Some(_)) => Ordering::Greater, // Some < None
        (Some(a), Some(b)) => a
            .watched_status
            .partial_cmp(&b.watched_status)
            .unwrap_or(Ordering::Equal),
        _ => Ordering::Equal, // Both None
    }
}
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
    take_opt: Option<usize>,
    skip_opt: Option<usize>,
    vec: Vec<RequestStatus>,
    all_requests: usize,
) -> RequestStatusWithRecordInfoVector {
    let final_results = match (take_opt, skip_opt) {
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
        all_requests: all_requests,
        requests: final_results.to_vec(),
    };

    return req_status_info;
}

pub fn sort_requests_vector(
    sort_by: SortableProps,
    mut vec: Vec<RequestStatus>,
) -> Vec<RequestStatus> {
    match sort_by {
        SortableProps::Name => vec.sort_unstable_by(|a, b| {
            a.media_info
                .title
                .partial_cmp(&b.media_info.title)
                .unwrap_or(Ordering::Equal)
        }),
        SortableProps::MediaType => vec.sort_unstable_by(|a, b| {
            compare_media_variants(
                &a.media_request.media.media_type,
                &b.media_request.media.media_type,
            )
        }),
        SortableProps::User => vec.sort_unstable_by(|a, b| {
            a.media_request
                .requested_by
                .username
                .partial_cmp(&b.media_request.requested_by.username)
                .unwrap_or(Ordering::Equal)
        }),
        SortableProps::Watched => vec.sort_unstable_by(|a, b| {
            compare_user_watch_history(&a.user_watch_history, &b.user_watch_history)
        }),
        SortableProps::RequestedDate => vec.sort_unstable_by(|a, b| {
            a.media_request
                .created_at
                .partial_cmp(&b.media_request.created_at)
                .unwrap_or(Ordering::Equal)
        }),
    };

    return vec;
}

pub async fn get_requests_and_update_cache(
    app_data: Data<AppData>,
    params: QueryParms,
) -> Result<RequestStatusWithRecordInfoVector, DeleterrError> {
    let cache = get_cached(&app_data);
    let requests = get_requests_from_cache_or_update_cache(cache, app_data, params.take).await?;

    // Get vector
    let mut vec: Vec<RequestStatus> = requests.requests.into_values().collect();
    // TODO: filter the vev here
    //Sort the vector
    vec = sort_requests_vector(params.sort_by.unwrap_or(SortableProps::RequestedDate), vec);

    let req_status_info =
        make_filtered_sized_vector_of_results(params.take, params.skip, vec, requests.all_requests);

    Ok(req_status_info)
}
