#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(test)]
#[tokio::test]
async fn show_with_status() {
    use crate::{
        deleterr::services::get_request_status_for_series,
        overseerr::models::{MediaRequest, OverseerrListResponse},
        sonarr::series::Series,
        tautulli::models::TautulliResponse,
    };

    let tau_resp = serde_json::from_str::<TautulliResponse>(TAU_HIST)
        .unwrap()
        .response
        .data
        .data;

    let ovr_resp =
        serde_json::from_str::<OverseerrListResponse<MediaRequest>>(OVERSEERR_REQ).unwrap();
    let sonarr_resp = serde_json::from_str::<Vec<Series>>(SONARR_RESP).unwrap();

    let media_request = &ovr_resp.results[0];
    let media_type = &media_request.media.media_type;

    let req_status =
        get_request_status_for_series(media_request, sonarr_resp.get(0).cloned(), tau_resp)
            .await
            .unwrap();

    println!("{:?}", req_status.watched);
    assert!(matches!(
        req_status.watched,
        crate::deleterr::watched::WatchedStatus::Watched
    ));
}

#[tokio::test]
async fn movie_with_status() {
    use crate::{
        deleterr::services::get_request_status_for_movie,
        overseerr::models::{MediaRequest, OverseerrListResponse},
        radarr::models::Movie,
        tautulli::models::TautulliResponse,
    };
    let tau_resp = serde_json::from_str::<TautulliResponse>(TAU_HIST_MOVIE)
        .unwrap()
        .response
        .data
        .data;

    let ovr_resp =
        serde_json::from_str::<OverseerrListResponse<MediaRequest>>(OVERSEERR_REQ_MOVIE).unwrap();
    let radarr_resp = serde_json::from_str::<Option<Movie>>(RADARR_RESP).unwrap();

    let media_request = &ovr_resp.results[0];
    let media_type = &media_request.media.media_type;

    let req_status = get_request_status_for_movie(media_request, radarr_resp, tau_resp)
        .await
        .unwrap();

    println!("{:?}", req_status.watched);
    assert!(matches!(
        req_status.watched,
        crate::deleterr::watched::WatchedStatus::Watched
    ));
}

const OVERSEERR_REQ_MOVIE: &str = r#"{
  "pageInfo": {
    "pages": 13,
    "pageSize": 10,
    "results": 127,
    "page": 1
  },
  "results": [
    {
      "id": 151,
      "status": 2,
      "createdAt": "2023-09-17T14:27:53.000Z",
      "updatedAt": "2023-09-17T14:27:53.000Z",
      "type": "movie",
      "is4k": false,
      "serverId": 0,
      "profileId": 4,
      "rootFolder": "/movies",
      "languageProfileId": null,
      "tags": [],
      "isAutoRequest": false,
      "requestedBy": {
        "permissions": 2,
        "id": 1,
        "email": "omarmir@gmail.com",
        "plexUsername": "omarshirazmir",
        "username": null,
        "recoveryLinkExpirationDate": null,
        "userType": 1,
        "plexId": 538225,
        "avatar": "https://plex.tv/users/xxxxx/avatar?c=1700712990",
        "movieQuotaLimit": null,
        "movieQuotaDays": null,
        "tvQuotaLimit": null,
        "tvQuotaDays": null,
        "createdAt": "2022-12-06T17:09:12.000Z",
        "updatedAt": "2023-11-23T04:16:30.000Z",
        "settings": {
          "id": 4,
          "locale": "",
          "region": null,
          "originalLanguage": null,
          "pgpKey": null,
          "discordId": null,
          "pushbulletAccessToken": null,
          "pushoverApplicationToken": null,
          "pushoverUserKey": null,
          "telegramChatId": null,
          "telegramSendSilently": null,
          "watchlistSyncMovies": null,
          "watchlistSyncTv": null,
          "notificationTypes": {
            "webpush": 4204,
            "email": 4196
          }
        },
        "requestCount": 15,
        "displayName": "omarshirazmir"
      },
      "modifiedBy": {
        "permissions": 2,
        "id": 1,
        "email": "omarmir@gmail.com",
        "plexUsername": "omarshirazmir",
        "username": null,
        "recoveryLinkExpirationDate": null,
        "userType": 1,
        "plexId": 538225,
        "avatar": "https://plex.tv/users/xxxxx/avatar?c=1700712990",
        "movieQuotaLimit": null,
        "movieQuotaDays": null,
        "tvQuotaLimit": null,
        "tvQuotaDays": null,
        "createdAt": "2022-12-06T17:09:12.000Z",
        "updatedAt": "2023-11-23T04:16:30.000Z",
        "settings": {
          "id": 4,
          "locale": "",
          "region": null,
          "originalLanguage": null,
          "pgpKey": null,
          "discordId": null,
          "pushbulletAccessToken": null,
          "pushoverApplicationToken": null,
          "pushoverUserKey": null,
          "telegramChatId": null,
          "telegramSendSilently": null,
          "watchlistSyncMovies": null,
          "watchlistSyncTv": null,
          "notificationTypes": {
            "webpush": 4204,
            "email": 4196
          }
        },
        "requestCount": 15,
        "displayName": "omarshirazmir"
      },
      "media": {
        "downloadStatus": [],
        "downloadStatus4k": [],
        "id": 899,
        "mediaType": "movie",
        "tmdbId": 330457,
        "tvdbId": null,
        "imdbId": null,
        "status": 5,
        "status4k": 1,
        "createdAt": "2023-09-17T14:27:53.000Z",
        "updatedAt": "2023-09-17T14:40:00.000Z",
        "lastSeasonChange": "2023-09-17T14:27:53.000Z",
        "mediaAddedAt": "2023-09-17T14:36:05.000Z",
        "serviceId": 0,
        "serviceId4k": null,
        "externalServiceId": 801,
        "externalServiceId4k": null,
        "externalServiceSlug": "330457",
        "externalServiceSlug4k": null,
        "ratingKey": "66340",
        "ratingKey4k": null,
        "seasons": [],
        "plexUrl": "https://app.plex.tv/desktop#!/server/c6915a82b298954d467be259581524395cd9ae78/details?key=%2Flibrary%2Fmetadata%2F66340",
        "iOSPlexUrl": "plex://preplay/?metadataKey=%2Flibrary%2Fmetadata%2F66340&server=c6915a82b298954d467be259581524395cd9ae78",
        "serviceUrl": "http://192.168.2.102:7878/movie/330457"
      },
      "seasons": [],
      "seasonCount": 0
    }
  ]
}"#;

const RADARR_RESP: &str = r#"{
  "title": "Frozen II",
  "originalTitle": "Frozen II",
  "originalLanguage": {
    "id": 1,
    "name": "English"
  },
  "alternateTitles": [
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Frozen 2",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8633
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Замръзналото кралство II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 29,
        "name": "Bulgarian"
      },
      "id": 8634
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "冰雪奇緣 II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8635
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Nữ Hoàng Băng Giá II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8636
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "겨울왕국 II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 21,
        "name": "Korean"
      },
      "id": 8637
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "La Reine des Neiges 2",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 2,
        "name": "French"
      },
      "id": 8638
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Die Eiskönigin II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 4,
        "name": "German"
      },
      "id": 8639
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Frost II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 15,
        "name": "Norwegian"
      },
      "id": 8640
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Lumekuninganna II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8641
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Karlar Ülkesi II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8642
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Snježno kraljevstvo II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8643
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Jégvarázs II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8644
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Frozen II Il Segreto di Arendelle",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 5,
        "name": "Italian"
      },
      "id": 8645
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "アナと雪の女王 II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 8,
        "name": "Japanese"
      },
      "id": 8646
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Regatul de Gheata II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8647
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Kraina Lodu II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8648
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Ladové Král'ovstvo II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 35,
        "name": "Slovak"
      },
      "id": 8649
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Ledove Království II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 25,
        "name": "Czech"
      },
      "id": 8650
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "La Reine des Neiges II",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8651
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "冰雪奇缘2",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 10,
        "name": "Chinese"
      },
      "id": 8652
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "فروزن ۲",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 33,
        "name": "Persian"
      },
      "id": 8653
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Die Eiskönigin 2",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8654
    },
    {
      "sourceType": "tmdb",
      "movieMetadataId": 436,
      "title": "Мұзды өлке 2",
      "sourceId": 0,
      "votes": 0,
      "voteCount": 0,
      "language": {
        "id": 1,
        "name": "English"
      },
      "id": 8655
    }
  ],
  "secondaryYearSourceId": 0,
  "sortTitle": "frozen ii",
  "sizeOnDisk": 29110860970,
  "status": "released",
  "overview": "Elsa, Anna, Kristoff and Olaf head far into the forest to learn the truth about an ancient mystery of their kingdom.",
  "inCinemas": "2019-11-20T00:00:00Z",
  "physicalRelease": "2020-02-25T00:00:00Z",
  "digitalRelease": "2020-02-11T00:00:00Z",
  "images": [
    {
      "coverType": "poster",
      "url": "/MediaCover/801/poster.jpg?lastWrite=638372852033243983",
      "remoteUrl": "https://image.tmdb.org/t/p/original/mINJaa34MtknCYl5AjtNJzWj8cD.jpg"
    },
    {
      "coverType": "fanart",
      "url": "/MediaCover/801/fanart.jpg?lastWrite=638372852033843974",
      "remoteUrl": "https://image.tmdb.org/t/p/original/xJWPZIYOEFIjZpBL7SVBGnzRYXp.jpg"
    }
  ],
  "website": "https://movies.disney.com/frozen-2",
  "year": 2019,
  "hasFile": true,
  "youTubeTrailerId": "vSKlICmmi98",
  "studio": "Walt Disney Pictures",
  "path": "/movies/Frozen II (2019)",
  "qualityProfileId": 4,
  "monitored": true,
  "minimumAvailability": "released",
  "isAvailable": true,
  "folderName": "/movies/Frozen II (2019)",
  "runtime": 103,
  "cleanTitle": "frozenii",
  "imdbId": "tt4520988",
  "tmdbId": 330457,
  "titleSlug": "330457",
  "certification": "PG",
  "genres": [
    "Family",
    "Animation",
    "Adventure"
  ],
  "tags": [],
  "added": "2023-09-17T14:27:53Z",
  "ratings": {
    "imdb": {
      "votes": 188460,
      "value": 6.8,
      "type": "user"
    },
    "tmdb": {
      "votes": 9155,
      "value": 7.269,
      "type": "user"
    },
    "metacritic": {
      "votes": 0,
      "value": 64,
      "type": "user"
    },
    "rottenTomatoes": {
      "votes": 0,
      "value": 77,
      "type": "user"
    }
  },
  "movieFile": {
    "movieId": 801,
    "relativePath": "Frozen II 2019.mkv",
    "path": "/movies/Frozen II (2019)/Frozen II 2019.mkv",
    "size": 29110860970,
    "dateAdded": "2023-09-17T14:33:56Z",
    "sceneName": "Frozen.II.2019.1080p.Remux.AVC.TrueHD.Atmos.7.1-playBD",
    "indexerFlags": 0,
    "quality": {
      "quality": {
        "id": 30,
        "name": "Remux-1080p",
        "source": "bluray",
        "resolution": 1080,
        "modifier": "remux"
      },
      "revision": {
        "version": 1,
        "real": 0,
        "isRepack": false
      }
    },
    "mediaInfo": {
      "audioBitrate": 0,
      "audioChannels": 7.1,
      "audioCodec": "TrueHD Atmos",
      "audioLanguages": "eng/eng/rum",
      "audioStreamCount": 3,
      "videoBitDepth": 8,
      "videoBitrate": 0,
      "videoCodec": "AVC",
      "videoDynamicRangeType": "",
      "videoFps": 23.976,
      "resolution": "1920x1080",
      "runTime": "1:43:13",
      "scanType": "Progressive",
      "subtitles": "eng/eng/fre/fre/ger/ger/rum"
    },
    "originalFilePath": "Frozen.II.2019.1080p.Remux.AVC.TrueHD.Atmos.7.1-playBD/23993fd3bf134bb19e43fafe205f03f1.mkv",
    "qualityCutoffNotMet": false,
    "languages": [
      {
        "id": 1,
        "name": "English"
      },
      {
        "id": 27,
        "name": "Romanian"
      }
    ],
    "releaseGroup": "playBD",
    "edition": "",
    "id": 4876
  },
  "collection": {
    "title": "Frozen Collection",
    "tmdbId": 386382,
    "monitored": false,
    "qualityProfileId": 0,
    "searchOnAdd": false,
    "minimumAvailability": "tba",
    "images": [],
    "added": "0001-01-01T05:17:00Z",
    "id": 0
  },
  "popularity": 137.801,
  "id": 801
}"#;

const SONARR_RESP: &str = r#"[
    {
      "title": "Little Demon",
      "alternateTitles": [],
      "sortTitle": "little demon",
      "status": "ended",
      "ended": true,
      "overview": "13 years after being impregnated by the Devil, a reluctant mother and her Antichrist daughter attempt to live an ordinary life in Delaware, but are constantly thwarted by monstrous forces, including Satan, who yearns for custody of his daughter's soul.",
      "previousAiring": "2022-10-21T02:00:00Z",
      "network": "FXX",
      "airTime": "22:00",
      "images": [
        {
          "coverType": "banner",
          "url": "/MediaCover/464/banner.jpg?lastWrite=638191521963058815",
          "remoteUrl": "https://artworks.thetvdb.com/banners/v4/series/381028/banners/6309fd456971e.jpg"
        },
        {
          "coverType": "poster",
          "url": "/MediaCover/464/poster.jpg?lastWrite=638191521963578807",
          "remoteUrl": "https://artworks.thetvdb.com/banners/v4/series/381028/posters/62fc16e55dab5.jpg"
        },
        {
          "coverType": "fanart",
          "url": "/MediaCover/464/fanart.jpg?lastWrite=638191521966178765",
          "remoteUrl": "https://artworks.thetvdb.com/banners/v4/series/381028/backgrounds/62fc17af19aa9.jpg"
        }
      ],
      "seasons": [
        {
          "seasonNumber": 1,
          "monitored": true,
          "statistics": {
            "previousAiring": "2022-10-21T02:00:00Z",
            "episodeFileCount": 10,
            "episodeCount": 10,
            "totalEpisodeCount": 10,
            "sizeOnDisk": 10316859742,
            "releaseGroups": [
              "NTb"
            ],
            "percentOfEpisodes": 100.0
          }
        }
      ],
      "year": 2022,
      "path": "/tv/Little Demon",
      "qualityProfileId": 7,
      "languageProfileId": 1,
      "seasonFolder": false,
      "monitored": true,
      "useSceneNumbering": false,
      "runtime": 24,
      "tvdbId": 381028,
      "tvRageId": 0,
      "tvMazeId": 47475,
      "firstAired": "2022-08-25T00:00:00Z",
      "seriesType": "standard",
      "cleanTitle": "littledemon",
      "imdbId": "tt12198014",
      "titleSlug": "little-demon",
      "rootFolderPath": "/tv/",
      "certification": "TV-MA",
      "genres": [
        "Animation",
        "Comedy"
      ],
      "tags": [],
      "added": "2023-05-08T14:16:35.730257Z",
      "ratings": {
        "votes": 0,
        "value": 0.0
      },
      "statistics": {
        "seasonCount": 1,
        "episodeFileCount": 10,
        "episodeCount": 10,
        "totalEpisodeCount": 10,
        "sizeOnDisk": 10316859742,
        "releaseGroups": [
          "NTb"
        ],
        "percentOfEpisodes": 100.0
      },
      "id": 464
    }
  ]"#;

const OVERSEERR_REQ: &str = r#"{
  "pageInfo": {
      "pages": 13,
      "pageSize": 10,
      "results": 127,
      "page": 1
  },
  "results": [
      {
          "id": 71,
          "status": 2,
          "createdAt": "2023-05-08T14:16:35.000Z",
          "updatedAt": "2023-05-08T14:16:35.000Z",
          "type": "tv",
          "is4k": false,
          "serverId": 0,
          "profileId": 7,
          "rootFolder": "/tv",
          "languageProfileId": 1,
          "tags": [],
          "isAutoRequest": false,
          "requestedBy": {
              "permissions": 2,
              "id": 1,
              "email": "test@test.com",
              "plexUsername": "testuser",
              "username": null,
              "recoveryLinkExpirationDate": null,
              "userType": 1,
              "plexId": 12345,
              "avatar": "https://plex.tv/users/xxxxxx/avatar?c=xxxxxx",
              "movieQuotaLimit": null,
              "movieQuotaDays": null,
              "tvQuotaLimit": null,
              "tvQuotaDays": null,
              "createdAt": "2022-12-06T17:09:12.000Z",
              "updatedAt": "2023-11-23T04:16:30.000Z",
              "settings": {
                  "id": 4,
                  "locale": "",
                  "region": null,
                  "originalLanguage": null,
                  "pgpKey": null,
                  "discordId": null,
                  "pushbulletAccessToken": null,
                  "pushoverApplicationToken": null,
                  "pushoverUserKey": null,
                  "telegramChatId": null,
                  "telegramSendSilently": null,
                  "watchlistSyncMovies": null,
                  "watchlistSyncTv": null,
                  "notificationTypes": {
                      "webpush": 4204,
                      "email": 4196
                  }
              },
              "requestCount": 15,
              "displayName": "testuser"
          },
          "modifiedBy": {
              "permissions": 2,
              "id": 1,
              "email": "test@test.com",
              "plexUsername": "testuser",
              "username": null,
              "recoveryLinkExpirationDate": null,
              "userType": 1,
              "plexId": 12345,
              "avatar": "https://plex.tv/users/xxxxxx/avatar?c=xxxxxx",
              "movieQuotaLimit": null,
              "movieQuotaDays": null,
              "tvQuotaLimit": null,
              "tvQuotaDays": null,
              "createdAt": "2022-12-06T17:09:12.000Z",
              "updatedAt": "2023-11-23T04:16:30.000Z",
              "settings": {
                  "id": 4,
                  "locale": "",
                  "region": null,
                  "originalLanguage": null,
                  "pgpKey": null,
                  "discordId": null,
                  "pushbulletAccessToken": null,
                  "pushoverApplicationToken": null,
                  "pushoverUserKey": null,
                  "telegramChatId": null,
                  "telegramSendSilently": null,
                  "watchlistSyncMovies": null,
                  "watchlistSyncTv": null,
                  "notificationTypes": {
                      "webpush": 4204,
                      "email": 4196
                  }
              },
              "requestCount": 15,
              "displayName": "testuser"
          },
          "media": {
              "downloadStatus": [],
              "downloadStatus4k": [],
              "id": 788,
              "mediaType": "tv",
              "tmdbId": 125392,
              "tvdbId": 381028,
              "imdbId": null,
              "status": 5,
              "status4k": 1,
              "createdAt": "2023-05-08T14:16:35.000Z",
              "updatedAt": "2023-05-08T14:20:00.000Z",
              "lastSeasonChange": "2023-05-08T14:20:00.057Z",
              "mediaAddedAt": "2023-05-08T14:17:07.000Z",
              "serviceId": 0,
              "serviceId4k": null,
              "externalServiceId": 464,
              "externalServiceId4k": null,
              "externalServiceSlug": "little-demon",
              "externalServiceSlug4k": null,
              "ratingKey": "64061",
              "ratingKey4k": null,
              "seasons": [
                  {
                      "id": 1279,
                      "seasonNumber": 1,
                      "status": 5,
                      "status4k": 1,
                      "createdAt": "2023-05-08T14:20:00.000Z",
                      "updatedAt": "2023-05-08T14:20:00.000Z"
                  }
              ]
          },
          "seasons": [
              {
                  "id": 30,
                  "seasonNumber": 1,
                  "status": 2,
                  "createdAt": "2023-05-08T14:16:35.000Z",
                  "updatedAt": "2023-05-08T14:16:35.000Z"
              }
          ],
          "seasonCount": 1
      }
  ]
}"#;

const TAU_HIST: &str = r#"{
  "response": {
    "result": "success",
    "message": null,
    "data": {
      "recordsFiltered": 10,
      "recordsTotal": 15164,
      "data": [
        {
          "reference_id": 13291,
          "row_id": 13291,
          "id": 13291,
          "date": 1692071250,
          "started": 1692071250,
          "stopped": 1692072740,
          "duration": 1490,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64069,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Village of the Found",
          "title": "Village of the Found",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 10,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-10-20",
          "guid": "plex://episode/6301d02ea4cb834bd4db933d",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "13291",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 13289,
          "row_id": 13289,
          "id": 13289,
          "date": 1692069760,
          "started": 1692069760,
          "stopped": 1692071249,
          "duration": 1489,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64065,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Wet Bodies",
          "title": "Wet Bodies",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 9,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-10-13",
          "guid": "plex://episode/6301d02ea4cb834bd4db932d",
          "transcode_decision": "direct play",
          "percent_complete": 99,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "13289",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 13165,
          "row_id": 13165,
          "id": 13165,
          "date": 1691557550,
          "started": 1691557550,
          "stopped": 1691558904,
          "duration": 1354,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64067,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Domestic Disturbance VIII",
          "title": "Domestic Disturbance VIII",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 8,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-10-06",
          "guid": "plex://episode/6301d02da4cb834bd4db9314",
          "transcode_decision": "direct play",
          "percent_complete": 99,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "13165",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 12869,
          "row_id": 12869,
          "id": 12869,
          "date": 1690510805,
          "started": 1690510805,
          "stopped": 1690512162,
          "duration": 1357,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64071,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Satan's Lot",
          "title": "Satan's Lot",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 7,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-09-29",
          "guid": "plex://episode/62e62b814a60bbaaecbbf369",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "12869",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 12561,
          "row_id": 12561,
          "id": 12561,
          "date": 1689385456,
          "started": 1689385456,
          "stopped": 1689386844,
          "duration": 1388,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64070,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - The Antichrist's Monster",
          "title": "The Antichrist's Monster",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 6,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-09-22",
          "guid": "plex://episode/62e62b814a60bbaaecbbf368",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "12561",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 12549,
          "row_id": 12549,
          "id": 12549,
          "date": 1689299652,
          "started": 1689299652,
          "stopped": 1689301036,
          "duration": 1384,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64068,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Night of the Leeches",
          "title": "Night of the Leeches",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 5,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-09-15",
          "guid": "plex://episode/62e62b804a60bbaaecbbf367",
          "transcode_decision": "direct play",
          "percent_complete": 98,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "12549",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 12547,
          "row_id": 12547,
          "id": 12547,
          "date": 1689298231,
          "started": 1689298231,
          "stopped": 1689299632,
          "duration": 1401,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64066,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Popularity: Origin of Evil",
          "title": "Popularity: Origin of Evil",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 4,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-09-08",
          "guid": "plex://episode/62e62b804a60bbaaecbbf366",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "12547",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 12532,
          "row_id": 12545,
          "id": 12545,
          "date": 1689297015,
          "started": 1689204313,
          "stopped": 1689298220,
          "duration": 1403,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64072,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Everybody's Dying for the Weekend",
          "title": "Everybody's Dying for the Weekend",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 3,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-09-01",
          "guid": "plex://episode/627fd43e1b84c2c682c354fc",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 2,
          "group_ids": "12532,12545",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 11438,
          "row_id": 11438,
          "id": 11438,
          "date": 1683600870,
          "started": 1683600870,
          "stopped": 1683602230,
          "duration": 1360,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64064,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - Possession Obsession",
          "title": "Possession Obsession",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 2,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-08-25",
          "guid": "plex://episode/627fd43e1b84c2c682c354fd",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "11438",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 11433,
          "row_id": 11436,
          "id": 11436,
          "date": 1683599668,
          "started": 1683598823,
          "stopped": 1683600849,
          "duration": 1564,
          "paused_counter": 304,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "episode",
          "rating_key": 64063,
          "parent_rating_key": 64062,
          "grandparent_rating_key": 64061,
          "full_title": "Little Demon - First Blood",
          "title": "First Blood",
          "parent_title": "Season 1",
          "grandparent_title": "Little Demon",
          "original_title": "",
          "year": "",
          "media_index": 1,
          "parent_media_index": 1,
          "thumb": "/library/metadata/64062/thumb/1683555428",
          "originally_available_at": "2022-08-25",
          "guid": "plex://episode/609d60bb98a87a002c15344a",
          "transcode_decision": "direct play",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 2,
          "group_ids": "11433,11436",
          "state": null,
          "session_key": null
        }
      ],
      "draw": 1,
      "filter_duration": "3 hrs 56 mins 30 secs",
      "total_duration": "3 hrs 56 mins 30 secs"
    }
  }
}"#;

const TAU_HIST_MOVIE: &str = r#"{
  "response": {
    "result": "success",
    "message": null,
    "data": {
      "recordsFiltered": 3,
      "recordsTotal": 15164,
      "data": [
        {
          "reference_id": 14424,
          "row_id": 14424,
          "id": 14424,
          "date": 1699219089,
          "started": 1699219089,
          "stopped": 1699224819,
          "duration": 5730,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Android",
          "product": "Plex for Android (TV)",
          "player": "SHIELD Android TV",
          "ip_address": "192.168.2.63",
          "live": 0,
          "machine_id": "4f60e410d5a5e6fd-com-plexapp-android",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "movie",
          "rating_key": 66340,
          "parent_rating_key": "",
          "grandparent_rating_key": "",
          "full_title": "Frozen II",
          "title": "Frozen II",
          "parent_title": "",
          "grandparent_title": "",
          "original_title": "",
          "year": 2019,
          "media_index": "",
          "parent_media_index": "",
          "thumb": "/library/metadata/66340/thumb/1697610956",
          "originally_available_at": "2019-11-20",
          "guid": "plex://movie/5d776b85594b2b001e6dc641",
          "transcode_decision": "direct play",
          "percent_complete": 93,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "14424",
          "state": null,
          "session_key": null
        },
        {
          "reference_id": 13968,
          "row_id": 13968,
          "id": 13968,
          "date": 1696187270,
          "started": 1696187270,
          "stopped": 1696193463,
          "duration": 6193,
          "paused_counter": 0,
          "user_id": 538225,
          "user": "xxxxx",
          "friendly_name": "xxxxx",
          "user_thumb": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
          "platform": "Roku",
          "product": "Plex for Roku",
          "player": "Roku Ultra",
          "ip_address": "192.168.2.50",
          "live": 0,
          "machine_id": "77fa23c2f71fb2668dc24f2e45485818",
          "location": "lan",
          "secure": 1,
          "relayed": 0,
          "media_type": "movie",
          "rating_key": 66340,
          "parent_rating_key": "",
          "grandparent_rating_key": "",
          "full_title": "Frozen II",
          "title": "Frozen II",
          "parent_title": "",
          "grandparent_title": "",
          "original_title": "",
          "year": 2019,
          "media_index": "",
          "parent_media_index": "",
          "thumb": "/library/metadata/66340/thumb/1694961368",
          "originally_available_at": "2019-11-20",
          "guid": "plex://movie/5d776b85594b2b001e6dc641",
          "transcode_decision": "transcode",
          "percent_complete": 100,
          "watched_status": 1,
          "group_count": 1,
          "group_ids": "13968",
          "state": null,
          "session_key": null
        }
      ],
      "draw": 1,
      "filter_duration": "4 hrs 48 mins 8 secs",
      "total_duration": "4 hrs 48 mins 8 secs"
    }
  }
}"#;
