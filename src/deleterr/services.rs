use super::models::{RequestStatus, RequestStatusWithRecordInfo};
use crate::common::models::{APIData, APIResponse, DeleterrError};
use crate::common::services::map_to_api_response;

use crate::overseerr::models::{MediaRequest, PageInfo};
use crate::tautulli::models::UserWatchHistory;

async fn get_os_requests() -> Result<(Vec<MediaRequest>, PageInfo), DeleterrError> {
    let os_requests = crate::os_serv::get_requests().await?.data;
    match os_requests {
        APIData::Success(data) => {
            let requests = data.results;
            let page_info = data.page_info;
            Ok((requests, page_info))
        }
        APIData::Failure(err) => Err(DeleterrError::new(err.as_str())),
    }
}

async fn get_tau_history_by_key_user(rating_key: &u64, user_id: &u64) -> Option<UserWatchHistory> {
    let tt_request = crate::tt_serv::get_item_history(
        rating_key.to_string().as_str(),
        user_id.to_string().as_str(),
    )
    .await
    .ok()?
    .data;

    let tt_matches = match tt_request {
        APIData::Success(tt_response) => tt_response.response.data.data,
        _ => None,
    };

    /*
     * We make sure that there is exactly one matched result since
     * we provided both a ratingKey and userId. If there is more than one result
     * then we did something wrong.
     */
    return match tt_matches {
        Some(histories) => {
            if histories.len() == 1 {
                Some(histories[0].clone())
            } else {
                None
            }
        }
        _ => None,
    };
}

pub async fn match_requests_to_watched(
    take: Option<usize>,
) -> Result<APIResponse<RequestStatusWithRecordInfo>, DeleterrError> {
    let chunk_size = take.unwrap_or(10);
    // ! Note that the default take is 10 at overseerr if unspecified!
    let (os_requests, page_info) = get_os_requests().await?;
    let mut matched_requests: Vec<RequestStatus> = Vec::with_capacity(page_info.results);

    for i in 0..5 {
        let media_request = &os_requests[i];
        let (media_type, req_id, rating_key, user_id) = (
            &media_request.media.media_type,
            &media_request.id,
            &media_request.media.rating_key,
            &media_request.requested_by.plex_id,
        );

        let media_info =
            Some(crate::overseerr::services::get_media_info(&media_type, &req_id).await?);
        let user_watch_history = match (rating_key, user_id) {
            (Some(rating_key), Some(user_id)) => {
                get_tau_history_by_key_user(&rating_key, &user_id).await
            }
            _ => None,
        };

        let request_status = RequestStatus {
            media_info,
            user_watch_history,
            media_request: media_request.clone(),
        };

        // Delay
        if i > 0 && (i % chunk_size) == 0 {
            println!("Sleeping for a sec");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }

        matched_requests.push(request_status)
    }

    let request_status_with_record_info = RequestStatusWithRecordInfo {
        all_requests: page_info.results,
        requests: matched_requests,
    };

    let api_response =
        map_to_api_response(request_status_with_record_info, 200, "Success".to_string()).await?;

    Ok(api_response)
}
