use crate::common::models::api::{APIStatus, ResponseCodeBasedAction};
use crate::common::models::deleterr_error::DeleterrError;
use crate::overseerr::models::MediaType;
use actix_web::web::Data;
use std::{cmp::Ordering, collections::HashMap};

use super::{
    models::{
        AppData, QueryParms, RequestStatus, RequestStatusWithRecordInfo,
        RequestStatusWithRecordInfoVector, SortableProps,
    },
    services::match_requests_to_watched,
    watched::WatchedStatus,
};

fn compare_media_variants(a: &MediaType, b: &MediaType, is_descending: bool) -> Ordering {
    let media_variants = if is_descending { (a, b) } else { (b, a) };
    match media_variants {
        (MediaType::Movie, MediaType::Movie) => Ordering::Equal,
        (MediaType::TV, MediaType::TV) => Ordering::Equal,
        (MediaType::Movie, MediaType::TV) => Ordering::Less,
        (MediaType::TV, MediaType::Movie) => Ordering::Greater,
    }
}

fn compare_user_watch_history(
    a: &WatchedStatus,
    b: &WatchedStatus,
    is_descending: bool,
) -> Ordering {
    let media_variants = if is_descending { (a, b) } else { (b, a) };
    match media_variants {
        (WatchedStatus::Watched, WatchedStatus::Watched) => Ordering::Equal,
        (WatchedStatus::Watched, _) => Ordering::Greater,

        (WatchedStatus::InProgress, WatchedStatus::Unwatched) => Ordering::Greater,
        (WatchedStatus::InProgress, WatchedStatus::InProgress) => Ordering::Equal,
        (WatchedStatus::InProgress, WatchedStatus::Watched) => Ordering::Less,

        (WatchedStatus::Unwatched, WatchedStatus::Unwatched) => Ordering::Equal,
        (WatchedStatus::Unwatched, _) => Ordering::Less,
    }
}
async fn get_data_update_cache(
    app_data: &Data<AppData>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let mut update_cache = app_data
        .request_cache
        .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
        .expect("Unable to access cache");

    let request_status_with_record_info = match_requests_to_watched().await?;

    *update_cache = Some(request_status_with_record_info.clone());
    Ok(request_status_with_record_info)
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

pub fn get_cached_record(app_data: &Data<AppData>, request_id: usize) -> Option<RequestStatus> {
    let record = get_cached(app_data);

    let resp = match record {
        Some(requests) => requests.requests.get(&request_id).cloned(),
        None => None,
    };

    resp
}

pub fn delete_cached_record(
    app_data: &Data<AppData>,
    request_id: usize,
) -> Result<ResponseCodeBasedAction, DeleterrError> {
    let mut update_cache = app_data
        .request_cache
        .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
        .map_err(|err| {
            DeleterrError::new(err.to_string().as_str())
                .add_prefix("Unable to access cache. Lock is poisoned.")
        })?;

    if let Some(del_cache) = update_cache.as_mut() {
        del_cache.requests.remove(&request_id);
    } else {
        return Err(DeleterrError::new(
            "Cache is empty. Maybe resync the cache first?",
        ));
    }
    Ok(ResponseCodeBasedAction {
        status: APIStatus::Success,
        success: true,
    })
}

pub async fn get_requests_from_cache_or_update_cache(
    cache: Option<RequestStatusWithRecordInfo>,
    app_data: Data<AppData>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let req = match cache {
        Some(cached) => cached,
        // Moved the data retrieval into the RWLock so that it cannot be obtained mutiple times at the same time
        None => get_data_update_cache(&app_data).await?,
    };

    Ok(req)
}

pub fn make_filtered_sized_vector_of_results(
    take_opt: Option<usize>,
    skip_opt: Option<usize>,
    vec: Vec<RequestStatus>,
    all_requests: usize,
) -> RequestStatusWithRecordInfoVector {
    let filtered_requests = vec.len();
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
        filtered_requests,
        all_requests,
        requests: final_results.to_vec(),
    };

    return req_status_info;
}

fn reversible_sort<T, F>(vec: &mut Vec<T>, compare: F, is_descending: bool)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    vec.sort_unstable_by(|a, b| {
        let cmp = compare(a, b);
        if is_descending {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

pub fn sort_requests_vector(
    sort_by: SortableProps,
    mut vec: Vec<RequestStatus>,
    is_descending: bool,
) -> Vec<RequestStatus> {
    match sort_by {
        SortableProps::Name => reversible_sort(
            &mut vec,
            |a, b| {
                a.media_info
                    .title
                    .partial_cmp(&b.media_info.title)
                    .unwrap_or(Ordering::Equal)
            },
            is_descending,
        ),
        SortableProps::MediaType => vec.sort_unstable_by(|a, b| {
            compare_media_variants(
                &a.media_request.media.media_type,
                &b.media_request.media.media_type,
                is_descending,
            )
        }),
        SortableProps::User => reversible_sort(
            &mut vec,
            |a, b| {
                a.media_request
                    .requested_by
                    .plex_id
                    .partial_cmp(&b.media_request.requested_by.plex_id)
                    .unwrap_or(Ordering::Equal)
            },
            is_descending,
        ),
        SortableProps::Watched => vec.sort_unstable_by(|a, b| {
            compare_user_watch_history(&a.watched, &b.watched, is_descending)
        }),
        SortableProps::RequestedDate => reversible_sort(
            &mut vec,
            |a, b| {
                a.media_request
                    .created_at
                    .partial_cmp(&b.media_request.created_at)
                    .unwrap_or(Ordering::Equal)
            },
            is_descending,
        ),
    };

    return vec;
}

fn get_filtered_vec(
    requests: HashMap<usize, RequestStatus>,
    search: Option<String>,
) -> Vec<RequestStatus> {
    let vec: Vec<RequestStatus> = match search {
        Some(search_str) => requests
            .into_values()
            .filter(|request_status| {
                request_status
                    .media_info
                    .title
                    .to_lowercase()
                    .contains(search_str.to_lowercase().as_str())
            })
            .collect(),
        None => requests.into_values().collect(),
    };

    return vec;
}

pub async fn get_requests_and_update_cache(
    app_data: Data<AppData>,
    params: QueryParms,
) -> Result<RequestStatusWithRecordInfoVector, DeleterrError> {
    let cache = get_cached(&app_data);
    let requests = get_requests_from_cache_or_update_cache(cache, app_data).await?;

    // Get the vector from the hashmap after being filtered
    let mut vec = get_filtered_vec(requests.requests, params.search);

    //Sort the vector descending by default
    vec = sort_requests_vector(
        params.sort_by.unwrap_or(SortableProps::RequestedDate),
        vec,
        params.is_descending.unwrap_or(true),
    );

    let req_status_info =
        make_filtered_sized_vector_of_results(params.take, params.skip, vec, requests.all_requests);

    Ok(req_status_info)
}
