use std::time::Duration;

use actix_web::web::Data;
use uuid::Uuid;

use crate::{
    common::{
        broadcast::{DeletionType, MessageType},
        models::deleterr_error::DeleterrError,
    },
    deleterr::{
        models::{AppData, MediaInfo, RequestStatus, SeriesDeletionEpisodes},
        watched::{SeasonWithStatus, WatchedChecker, WatchedStatus},
    },
    overseerr::models::MediaRequest,
    sonarr::series::Series,
    tautulli::user_watch_history::{ConvertToHashMapBySeason, UserWatchHistory},
};

pub fn get_request_status_for_series(
    media_request: &MediaRequest,
    sonarr_series: Option<Series>,
    tau_hist: Option<Vec<UserWatchHistory>>,
) -> Result<RequestStatus, DeleterrError> {
    let (season_status, watched) = {
        let tau_history = tau_hist.hashmap_seasons();
        let mut seasons_with_status = vec![];
        let series_map = Series::hashmap_seasons(&sonarr_series);

        for season in &media_request.seasons {
            let series_season = series_map.get(&season.season_number);
            let watched_statuses = tau_history.get(&season.season_number);
            let season_with_status =
                SeasonWithStatus::from_series(season, watched_statuses, series_season);
            seasons_with_status.push(season_with_status)
        }

        let watched = seasons_with_status.is_watched(*&media_request.seasons.len());

        (seasons_with_status, watched)
    };

    let media_info = MediaInfo::from_sonarr(sonarr_series.clone());

    let mut request_status = RequestStatus {
        media_info,
        season_status,
        media_request: media_request.clone(),
        watched,
    };

    // We can make this a lot more efficient - fine for now
    // TODO: Doing this manually for now, but it should be done via setting later right now we are setting watched seasons by latest watched/In progress season

    let cloned_seasons = request_status.season_status.clone(); // Don't want to modify as I iterate, so a clone is made

    for season in &mut request_status.season_status {
        if let Some(season_number) = season.season_number {
            let next_season = cloned_seasons
                .iter()
                .find(|curr_season| curr_season.season_number == Some(season_number + 1));

            if let Some(sesn) = next_season {
                if sesn.watched == WatchedStatus::InProgress
                    || sesn.watched == WatchedStatus::Watched
                {
                    season.watched = WatchedStatus::Watched
                };
            }
        }
    }

    request_status.watched = request_status
        .season_status
        .is_watched(*&media_request.seasons.len());

    Ok(request_status)
}

/// Looks at the current App State and calculates which episode files need deletion and if it the full request
/// that has been watched.
///
/// # Arguments:
///
/// * `request_id`: Used to identify a specific request for which we need to figure out watched seasons.
/// * `app_data`: The app state of type [AppData]
///
/// # Returns:
///
/// `Result` containing a list of episode file ids for deletion and a flag that indicates if all seasons are watched
/// which we can use to determine if we are OK to delete the entire request.
///
/// `Result` containing a [ResponseCodeBasedAction] or [DeleterrError] if there was an error during the process.
pub async fn get_watched_seasons_episodes(
    app_data: &Data<AppData>,
    request_id: &usize,
) -> Result<SeriesDeletionEpisodes, DeleterrError> {
    // Get the record from the cache
    let request = app_data.get_request(request_id).ok_or(DeleterrError::new(
        "No request found! You may need to resync the services",
    ))?;

    // Make sure we have the series ID in the request cache
    let series_id = request
        .media_request
        .media
        .external_service_id
        .ok_or(DeleterrError::new(
            "No ID found in Sonarr. Manual deletion required.",
        ))?;

    // Get a vec of watched season numbers
    let watched_seasons: Vec<usize> = request
        .season_status
        .iter()
        .filter(|season| season.watched == WatchedStatus::Watched)
        .filter_map(|season| season.season_number)
        .collect();

    // We need to know if we are deleting the request all together or not - if all requested seasons watched then yes, otherwise no
    let is_request_fully_watched = watched_seasons.len() == request.media_request.seasons.len();

    // Get all episodes for a series
    let episode_files = crate::sonarr::services::get_episode_files(series_id)
        .await?
        .ok_or(DeleterrError::new(
            "No episode files found for this series in Sonarr.",
        ))?;

    // Get a vec of file ids for all episodes inside the watched season
    let watched_episodes_for_seasons: Vec<usize> = episode_files
        .iter()
        .filter(|file| watched_seasons.contains(&file.season_number))
        .map(|watched_eps| watched_eps.id)
        .collect();

    Ok(SeriesDeletionEpisodes {
        episodes: watched_episodes_for_seasons,
        request_fully_watched: is_request_fully_watched,
    })
}

async fn delete_episodes_from_sonarr(
    app_data: &Data<AppData>,
    episodes: Vec<usize>,
) -> Result<(), DeleterrError> {
    let broadcaster = app_data.broadcaster.clone();

    if episodes.len() == 0 {
        return Err(DeleterrError::new(
            "No episodes found for request to delete",
        ));
    }

    let sonarr_response = crate::sonarr::services::delete_episodes(episodes);

    tokio::pin!(sonarr_response); // Pin the future so we can poll it

    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            result = &mut sonarr_response => {
                match result {
                    Ok(_) => {
                        broadcaster.broadcast(MessageType::Delete(DeletionType::Sonarr)).await;
                        return Ok(())
                    },
                    Err(err) => {
                        broadcaster.broadcast(MessageType::Error(err.to_string())).await;
                        return Err(err);
                    },
                }

            }
            _ = interval.tick() => {
                broadcaster.broadcast(MessageType::Waiting).await;
            }
        }
    }
}

/**
*     let request_info = app_data.get_request(request_id);

   let ended = match request_info {
       Some(req) => req.media_info.ended,
       None => None,
   };
*/

pub async fn delete_watched_seasons_and_possibly_request(
    app_data: Data<AppData>,
    request_id: usize,
    client_id: Uuid,
) {
    // ! We need to check if the index is being built right now
    // ! We also need to check:
    // * If the request is fully available
    // * if the request is fully watched
    // Only then can you delete the request.
    let broadcaster = app_data.broadcaster.clone();

    let episodes_for_deletion = get_watched_seasons_episodes(&app_data, &request_id)
        .await
        .unwrap_or(SeriesDeletionEpisodes::default());

    let episode_delete = delay_ten_seconds_for_test().await;
    //delete_episodes_from_sonarr(&app_data, episodes_for_deletion.episodes).await;

    // If not able to to delete from Sonarr then bail early.
    if let Err(_) = episode_delete {
        return;
    }

    match (episode_delete, episodes_for_deletion.request_fully_watched) {
        (Ok(_), true) => {
            let os_del = delay_ten_seconds_for_test().await;
            //crate::overseerr::services::delete_request(request_id.to_string().as_str()).await;
            match os_del {
                Ok(_) => {
                    broadcaster
                        .broadcast(MessageType::Delete(DeletionType::Overseer))
                        .await;
                    let cache_del = delay_ten_seconds_for_test().await; //app_data.delete_request(&request_id);

                    match cache_del {
                        Ok(_) => {
                            broadcaster
                                .broadcast(MessageType::Delete(DeletionType::Cache))
                                .await;
                            broadcaster.broadcast(MessageType::Complete).await;
                        }
                        Err(err) => {
                            broadcaster.broadcast(
                            MessageType::Error(
                                err.add_prefix("Deleted in Sonarr and Overseer but unable to remove from local index. Manually trigger refresh to resolve.").to_string()
                            )
                        ).await;
                        }
                    }
                }
                Err(err) => {
                    broadcaster.broadcast(
                        MessageType::Error(
                            err.add_prefix("Deleted in Sonarr but was unable to delete in Overseer or the local index.").to_string()
                        )
                    ).await;
                }
            }
        }
        _ => (),
    }

    broadcaster.close_client(client_id);
}

async fn delay_ten_seconds_for_test() -> Result<(), DeleterrError> {
    println!("Starting delay...");
    tokio::time::sleep(Duration::from_secs(12)).await;
    println!("10 seconds have passed.");

    Ok(())
}
