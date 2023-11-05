use crate::{
    common::models::DeleterrError, overseerr::models::MediaType, tautulli::models::UserWatchHistory,
};
use actix_web::web::Data;
use std::{cmp::Ordering, collections::HashMap};

use super::{
    models::{
        AppData, QueryParms, RequestStatus, RequestStatusWithRecordInfo,
        RequestStatusWithRecordInfoVector, SortableProps,
    },
    services::match_requests_to_watched,
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
    a: &Option<UserWatchHistory>,
    b: &Option<UserWatchHistory>,
    is_descending: bool,
) -> Ordering {
    let media_variants = if is_descending { (a, b) } else { (b, a) };
    match media_variants {
        (Some(a), Some(b)) => a
            .watched_status
            .partial_cmp(&b.watched_status)
            .unwrap_or(Ordering::Equal),
        (None, Some(_)) => Ordering::Less,    // None < Some
        (Some(_), None) => Ordering::Greater, // Some < None
        _ => Ordering::Equal,                 // Both None
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
            compare_user_watch_history(&a.user_watch_history, &b.user_watch_history, is_descending)
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
    let requests = get_requests_from_cache_or_update_cache(cache, app_data, params.take).await?;

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
