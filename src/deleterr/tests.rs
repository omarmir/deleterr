#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(test)]
#[tokio::test]
async fn all_episodes_watched() {
    use crate::{
        deleterr::services::get_request_status,
        overseerr::models::{MediaInfo, MediaRequest, OverseerrListResponse},
        sonarr::models::Episode,
        tautulli::models::TautulliResponse,
    };

    let tau_resp = serde_json::from_str::<TautulliResponse>(TAU_HIST)
        .unwrap()
        .response
        .data
        .data;
    let ovr_resp =
        serde_json::from_str::<OverseerrListResponse<MediaRequest>>(OVERSEERR_REQ).unwrap();
    let media_info = serde_json::from_str::<MediaInfo>(MEDIA_INFO).unwrap();
    let sonarr_resp = serde_json::from_str::<Vec<Episode>>(SONARR_RESP).unwrap();

    let media_request = &ovr_resp.results[0];
    let media_type = &media_request.media.media_type;

    let req_status = get_request_status(
        media_type,
        media_info,
        media_request,
        Some(sonarr_resp),
        tau_resp,
    )
    .await
    .unwrap();

    assert!(matches!(
        req_status.watched,
        crate::deleterr::watched::WatchedStatus::Watched
    ));
}

const MEDIA_INFO: &str = r#"{
  "createdBy": [
      {
          "id": 1210611,
          "credit_id": "609b6af828d7fe006e3fd173",
          "name": "Darcy Fowler",
          "gender": 0,
          "profile_path": null
      },
      {
          "id": 1264745,
          "credit_id": "609b6aff7646fd0040bf26b7",
          "name": "Seth Kirschner",
          "gender": 0,
          "profile_path": "/pOMdxNJyxXh9OGlp7fFSX9hXeLn.jpg"
      },
      {
          "id": 2118124,
          "credit_id": "609b6b04fba6250059c3604e",
          "name": "Kieran Valla",
          "gender": 2,
          "profile_path": null
      }
  ],
  "episodeRunTime": [],
  "firstAirDate": "2022-08-25",
  "genres": [
      {
          "id": 16,
          "name": "Animation"
      },
      {
          "id": 35,
          "name": "Comedy"
      }
  ],
  "relatedVideos": [],
  "homepage": "https://www.fxnetworks.com/shows/little-demon",
  "id": 125392,
  "inProduction": false,
  "languages": [
      "en"
  ],
  "lastAirDate": "2022-10-20",
  "name": "Little Demon",
  "networks": [
      {
          "id": 1035,
          "name": "FXX",
          "originCountry": "US",
          "logoPath": "/hDLXRZMBOCbpVYpkBbIlLvMXgdX.png"
      }
  ],
  "numberOfEpisodes": 10,
  "numberOfSeasons": 1,
  "originCountry": [
      "US"
  ],
  "originalLanguage": "en",
  "originalName": "Little Demon",
  "tagline": "Satan's teen daughter.",
  "overview": "13 years after being impregnated by Satan, a reluctant mother, Laura, and her Antichrist daughter, Chrissy, attempt to live an ordinary life in Delaware, but are constantly thwarted by monstrous forces, including Satan, who yearns for custody of his daughter's soul.",
  "popularity": 26.515,
  "productionCompanies": [
      {
          "id": 15990,
          "name": "FX Productions",
          "originCountry": "US",
          "logoPath": "/5cT4zwHA66uNAr2p3CcBDLddXu2.png"
      },
      {
          "id": 81667,
          "name": "ShadowMachine",
          "originCountry": "US",
          "logoPath": "/Am06MBFNH0OLe3m5MEhdyIFeYWh.png"
      },
      {
          "id": 179256,
          "name": "Jersey Films 2nd Avenue",
          "originCountry": "US",
          "logoPath": null
      }
  ],
  "productionCountries": [
      {
          "iso_3166_1": "US",
          "name": "United States of America"
      }
  ],
  "contentRatings": {
      "results": [
          {
              "descriptors": [],
              "iso_3166_1": "US",
              "rating": "TV-MA"
          },
          {
              "descriptors": [],
              "iso_3166_1": "BR",
              "rating": "16"
          },
          {
              "descriptors": [],
              "iso_3166_1": "SG",
              "rating": "R21"
          }
      ]
  },
  "spokenLanguages": [
      {
          "englishName": "English",
          "iso_639_1": "en",
          "name": "English"
      }
  ],
  "seasons": [
      {
          "airDate": "2022-08-25",
          "episodeCount": 10,
          "id": 194520,
          "name": "Season 1",
          "overview": "",
          "seasonNumber": 1,
          "posterPath": "/2lyFPOe6JScMBiLQuMtf3pPZxAu.jpg"
      }
  ],
  "status": "Canceled",
  "type": "Scripted",
  "voteAverage": 7.789,
  "voteCount": 64,
  "backdropPath": "/A7pq4B0uCPCLvk1EPFKQgJZQoVG.jpg",
  "lastEpisodeToAir": {
      "id": 3909377,
      "airDate": "2022-10-20",
      "episodeNumber": 10,
      "name": "Village of the Found",
      "overview": "Chrissy meets some family. Laura and Satan traverse a deadly maze. Bennigan gets a proposition.",
      "productionCode": "",
      "seasonNumber": 1,
      "showId": 125392,
      "voteAverage": 0,
      "stillPath": "/t9bhPfv8V58TJKeMRVL5DZJVVHG.jpg"
  },
  "posterPath": "/2lyFPOe6JScMBiLQuMtf3pPZxAu.jpg",
  "credits": {
      "cast": [
          {
              "character": "Laura Feinberg (voice)",
              "creditId": "609b70f49ca7590040be78e3",
              "id": 119592,
              "name": "Aubrey Plaza",
              "order": 0,
              "gender": 1,
              "profilePath": "/6qeCvOF1K88nKqZfzaWYESo0CwW.jpg"
          },
          {
              "character": "Chrissy Feinberg (voice)",
              "creditId": "609b70ec126ec3002aed885c",
              "id": 139310,
              "name": "Lucy DeVito",
              "order": 1,
              "gender": 1,
              "profilePath": "/jlLyxmuAe56QuOL02D8zfqpl1Bn.jpg"
          },
          {
              "character": "Satan (voice)",
              "creditId": "609b70dd963864005b788e58",
              "id": 518,
              "name": "Danny DeVito",
              "order": 2,
              "gender": 2,
              "profilePath": "/gNHF2SNXFFCRqwIQ2Xv6r6aV6UD.jpg"
          },
          {
              "character": "Bennigan (voice)",
              "creditId": "651373840745e1011c273433",
              "id": 1370567,
              "name": "Eugene Cordero",
              "order": 508,
              "gender": 2,
              "profilePath": "/waruLSR8lXBjhAFL0J6ihuVY62d.jpg"
          },
          {
              "character": "iCal / Party Goer (voice)",
              "creditId": "649bff3f963864011da8cd13",
              "id": 2342557,
              "name": "Laci Mosley",
              "order": 505,
              "gender": 1,
              "profilePath": "/2ZLFVsSkPkeQKADVZqEE9ZWQXwm.jpg"
          },
          {
              "character": "Unshaven Man (voice)",
              "creditId": "6410d928b4224200826ed512",
              "id": 335,
              "name": "Michael Shannon",
              "order": 500,
              "gender": 2,
              "profilePath": "/6mMczfjM8CiS1WuBOgo5Xom1TcR.jpg"
          },
          {
              "character": "Queen Inichoochiama (voice)",
              "creditId": "6410d939c390c5007f17d4c0",
              "id": 1029622,
              "name": "D.J. \"Shangela\" Pierce",
              "order": 501,
              "gender": 2,
              "profilePath": "/kHMN4zTZuTA6HITg2VlOxeHtcQo.jpg"
          },
          {
              "character": "Game Show Host (voice)",
              "creditId": "6410d947b4224200b9737057",
              "id": 1100,
              "name": "Arnold Schwarzenegger",
              "order": 502,
              "gender": 2,
              "profilePath": "/zEMhugsgXIpnQqO31GpAJYMUZZ1.jpg"
          }
      ],
      "crew": [
          {
              "creditId": "609b71379638640042befe60",
              "department": "Production",
              "id": 57194,
              "job": "Executive Producer",
              "name": "Dan Harmon",
              "gender": 2,
              "profilePath": "/gDwFosoyPTd0pmnKParzGj3kaMg.jpg"
          },
          {
              "creditId": "609b715c9824c800571b5ea9",
              "department": "Production",
              "id": 119592,
              "job": "Executive Producer",
              "name": "Aubrey Plaza",
              "gender": 1,
              "profilePath": "/6qeCvOF1K88nKqZfzaWYESo0CwW.jpg"
          },
          {
              "creditId": "609b71769824c8007a26824a",
              "department": "Production",
              "id": 1210611,
              "job": "Executive Producer",
              "name": "Darcy Fowler",
              "gender": 0,
              "profilePath": null
          },
          {
              "creditId": "609b717ec51acd003fb743a5",
              "department": "Production",
              "id": 1264745,
              "job": "Executive Producer",
              "name": "Seth Kirschner",
              "gender": 0,
              "profilePath": "/pOMdxNJyxXh9OGlp7fFSX9hXeLn.jpg"
          },
          {
              "creditId": "609b71867ecd28002afaf670",
              "department": "Production",
              "id": 2118124,
              "job": "Executive Producer",
              "name": "Kieran Valla",
              "gender": 2,
              "profilePath": null
          },
          {
              "creditId": "609b719c4202280029b104fd",
              "department": "Production",
              "id": 518,
              "job": "Executive Producer",
              "name": "Danny DeVito",
              "gender": 2,
              "profilePath": "/gNHF2SNXFFCRqwIQ2Xv6r6aV6UD.jpg"
          },
          {
              "creditId": "609b71a567203d0058aecbf8",
              "department": "Production",
              "id": 1286468,
              "job": "Executive Producer",
              "name": "Jake DeVito",
              "gender": 0,
              "profilePath": null
          },
          {
              "creditId": "609b71adc51acd006dbe5c47",
              "department": "Production",
              "id": 139310,
              "job": "Executive Producer",
              "name": "Lucy DeVito",
              "gender": 1,
              "profilePath": "/jlLyxmuAe56QuOL02D8zfqpl1Bn.jpg"
          },
          {
              "creditId": "609b71c62a210c00583ae938",
              "department": "Production",
              "id": 1853062,
              "job": "Executive Producer",
              "name": "Monica Mitchell",
              "gender": 0,
              "profilePath": null
          },
          {
              "creditId": "609b71ce2222f6006fe6e4d4",
              "department": "Production",
              "id": 69092,
              "job": "Executive Producer",
              "name": "Corey Campodonico",
              "gender": 2,
              "profilePath": null
          },
          {
              "creditId": "609b71d628d7fe00577240ab",
              "department": "Production",
              "id": 69089,
              "job": "Executive Producer",
              "name": "Alexander Bulkley",
              "gender": 2,
              "profilePath": null
          },
          {
              "creditId": "609b71ea126ec300404dccd7",
              "department": "Production",
              "id": 1588012,
              "job": "Producer",
              "name": "Steve Levy",
              "gender": 2,
              "profilePath": "/fXWLd5bk35p8QC64k4Ygy34yWn4.jpg"
          }
      ]
  },
  "externalIds": {
      "facebookId": "LittleDemonFX",
      "freebaseId": null,
      "freebaseMid": null,
      "imdbId": "tt12198014",
      "instagramId": "littledemonfx",
      "tvdbId": 381028,
      "tvrageId": null,
      "twitterId": "LittleDemonFX"
  },
  "keywords": [
      {
          "id": 10138,
          "name": "satan"
      },
      {
          "id": 8685,
          "name": "anti-christ"
      },
      {
          "id": 10809,
          "name": "teenage girl"
      },
      {
          "id": 15001,
          "name": "demon"
      },
      {
          "id": 161919,
          "name": "adult animation"
      }
  ],
  "mediaInfo": {
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
      "requests": [
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
                  "ratingKey4k": null
              },
              "requestedBy": {
                  "permissions": 2,
                  "id": 1,
                  "email": "test@test.com",
                  "plexUsername": "testuser",
                  "username": null,
                  "recoveryLinkExpirationDate": null,
                  "userType": 1,
                  "plexId": "xxxxxxx",
                  "avatar": "https://plex.tv/users/xxxxx/avatar?c=xxxxx",
                  "movieQuotaLimit": null,
                  "movieQuotaDays": null,
                  "tvQuotaLimit": null,
                  "tvQuotaDays": null,
                  "createdAt": "2022-12-06T17:09:12.000Z",
                  "updatedAt": "2023-11-23T04:16:30.000Z",
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
                  "plexId": "xxxxxxx",
                  "avatar": "https://plex.tv/users/xxxxxx/avatar?c=xxxxxx",
                  "movieQuotaLimit": null,
                  "movieQuotaDays": null,
                  "tvQuotaLimit": null,
                  "tvQuotaDays": null,
                  "createdAt": "2022-12-06T17:09:12.000Z",
                  "updatedAt": "2023-11-23T04:16:30.000Z",
                  "requestCount": 15,
                  "displayName": "testuser"
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
      ],
      "issues": [],
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
  "watchProviders": [
      {
          "iso_3166_1": "CA",
          "link": "https://www.themoviedb.org/tv/125392-little-demon/watch?locale=CA",
          "buy": [
              {
                  "displayPriority": 6,
                  "logoPath": "/peURlLlr8jggOwK53fJ5wdQl05y.jpg",
                  "id": 2,
                  "name": "Apple TV"
              },
              {
                  "displayPriority": 8,
                  "logoPath": "/tbEdFQDwx5LEVr8WpSeXQSIirVq.jpg",
                  "id": 3,
                  "name": "Google Play Movies"
              }
          ],
          "flatrate": []
      },
      {
          "iso_3166_1": "US",
          "link": "https://www.themoviedb.org/tv/125392-little-demon/watch?locale=US",
          "buy": [
              {
                  "displayPriority": 4,
                  "logoPath": "/peURlLlr8jggOwK53fJ5wdQl05y.jpg",
                  "id": 2,
                  "name": "Apple TV"
              },
              {
                  "displayPriority": 15,
                  "logoPath": "/5NyLm42TmCqCMOZFvH4fcoSNKEW.jpg",
                  "id": 10,
                  "name": "Amazon Video"
              },
              {
                  "displayPriority": 16,
                  "logoPath": "/tbEdFQDwx5LEVr8WpSeXQSIirVq.jpg",
                  "id": 3,
                  "name": "Google Play Movies"
              },
              {
                  "displayPriority": 42,
                  "logoPath": "/21dEscfO8n1tL35k4DANixhffsR.jpg",
                  "id": 7,
                  "name": "Vudu"
              }
          ],
          "flatrate": []
      }
  ]
}"#;

const SONARR_RESP: &str = r#"[
  {
    "seriesId": 464,
    "tvdbId": 9163446,
    "episodeFileId": 38695,
    "seasonNumber": 1,
    "episodeNumber": 1,
    "title": "First Blood",
    "airDate": "2022-08-25",
    "airDateUtc": "2022-08-26T02:00:00Z",
    "overview": "Chrissy Feinberg’s first day of seventh grade goes south when she discovers she’s the Antichrist.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49906
  },
  {
    "seriesId": 464,
    "tvdbId": 9280563,
    "episodeFileId": 38697,
    "seasonNumber": 1,
    "episodeNumber": 2,
    "title": "Possession Obsession",
    "airDate": "2022-08-25",
    "airDateUtc": "2022-08-26T02:24:00Z",
    "overview": "Chrissy gets carried away with possessing people and an old associate seeks revenge on Satan.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49907
  },
  {
    "seriesId": 464,
    "tvdbId": 9283735,
    "episodeFileId": 38704,
    "seasonNumber": 1,
    "episodeNumber": 3,
    "title": "Everybody's Dying for the Weekend",
    "airDate": "2022-09-01",
    "airDateUtc": "2022-09-02T02:00:00Z",
    "overview": "Satan and Chrissy enter an otherworldly game show. Laura and Darlene’s girl’s night gets cray cray.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49908
  },
  {
    "seriesId": 464,
    "tvdbId": 9283736,
    "episodeFileId": 38699,
    "seasonNumber": 1,
    "episodeNumber": 4,
    "title": "Popularity: Origin of Evil",
    "airDate": "2022-09-08",
    "airDateUtc": "2022-09-09T02:00:00Z",
    "overview": "Chrissy cooks up some creepypasta with the cool girls. Laura regresses to save her.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49909
  },
  {
    "seriesId": 464,
    "tvdbId": 9283737,
    "episodeFileId": 38700,
    "seasonNumber": 1,
    "episodeNumber": 5,
    "title": "Night of the Leeches",
    "airDate": "2022-09-15",
    "airDateUtc": "2022-09-16T02:00:00Z",
    "overview": "Laura uses magic to pretend she’s a normal mom. Satan bros out at a monstrous bachelor party.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49910
  },
  {
    "seriesId": 464,
    "tvdbId": 9283738,
    "episodeFileId": 38702,
    "seasonNumber": 1,
    "episodeNumber": 6,
    "title": "The Antichrist's Monster",
    "airDate": "2022-09-22",
    "airDateUtc": "2022-09-23T02:00:00Z",
    "overview": "Chrissy learns her best friend has a secret. Laura and Satan unite for the sake of their old dog.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49911
  },
  {
    "seriesId": 464,
    "tvdbId": 9283739,
    "episodeFileId": 38703,
    "seasonNumber": 1,
    "episodeNumber": 7,
    "title": "Satan's Lot",
    "airDate": "2022-09-29",
    "airDateUtc": "2022-09-30T02:00:00Z",
    "overview": "Chrissy gets an afterlife lesson from Satan. Laura helps Darlene with a dead ex-husband problem.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49912
  },
  {
    "seriesId": 464,
    "tvdbId": 9314845,
    "episodeFileId": 38698,
    "seasonNumber": 1,
    "episodeNumber": 8,
    "title": "Domestic Disturbance VIII",
    "airDate": "2022-10-06",
    "airDateUtc": "2022-10-07T02:00:00Z",
    "overview": "Laura and Chrissy’s disagreement over a dirty dish spirals into all out mother-daughter war.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49913
  },
  {
    "seriesId": 464,
    "tvdbId": 9314846,
    "episodeFileId": 38696,
    "seasonNumber": 1,
    "episodeNumber": 9,
    "title": "Wet Bodies",
    "airDate": "2022-10-13",
    "airDateUtc": "2022-10-14T02:00:00Z",
    "overview": "Laura rediscovers herself at Darlene’s beach house while Chrissy goes on a metaphysical Rumspringa.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49914
  },
  {
    "seriesId": 464,
    "tvdbId": 9314847,
    "episodeFileId": 38701,
    "seasonNumber": 1,
    "episodeNumber": 10,
    "title": "Village of the Found",
    "airDate": "2022-10-20",
    "airDateUtc": "2022-10-21T02:00:00Z",
    "overview": "Chrissy meets some family. Laura and Satan traverse a deadly maze. Bennigan gets a proposition.",
    "hasFile": true,
    "monitored": true,
    "unverifiedSceneNumbering": false,
    "id": 49915
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
          "recordsTotal": 14967,
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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
                  "user_id": 12345,
                  "user": "testuser",
                  "friendly_name": "testuser",
                  "user_thumb": "https://plex.tv/users/xxxxxx/avatar?c=1701351546",
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