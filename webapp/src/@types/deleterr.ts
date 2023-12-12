export interface APIResponse<T> {
  success: boolean
  data?: T
  error_msg?: string
}
export interface MediaRequest {
  id: number
  status: number
  requestedBy: User
  media: Media
  createdAt: string
}

export interface Media {
  id: number
  mediaType?: MediaType
  tmdbId?: number
  tvdbId?: number
  ratingKey?: number
  status: number
  externalServiceId: number
}

export type MediaType = 'movie' | 'tv'

export interface RequestStatus {
  mediaRequest: MediaRequest
  seasonStatus: SeasonStatus[]
  mediaInfo?: MediaInfo
  watched: WatchedStatus
}

export interface RequestStatusWithRecordInfo {
  allRequests: number
  filteredRequests: number
  requests: RequestStatus[]
}

export interface SeasonStatus {
  seasonNumber?: number
  reqStatus: number
  watched: WatchedStatus
  totalItems: number
  lastSeasonWithFiles: boolean
  episodesWithStatus: EpisodeWithStatus
}

export enum WatchedStatus {
  Unwatched = 'Unwatched',
  InProgress = 'In Progress',
  Watched = 'Watched',
}

export interface EpisodeWithStatus {
  externalServiceId: number
  fileId?: number
  watchedStatus: number //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
  episodeNumber: number
  seasonNumber: number
}

export type MediaExemption = { [key: string]: number }

export type SingleMediaExeption = [requestId: number, tmdbId: number]

export enum APIStatus {
  Success,
  WrongAPIKey,
  Other,
  NotFound,
}

export interface ResponseCodeBasedAction {
  status: APIStatus
  success: boolean
}

export interface MovieDeletionRequest {
  radarrResponse?: ResponseCodeBasedAction
  overseerrResponse?: ResponseCodeBasedAction
}

export interface User {
  id: number
  username?: string
  userType: number
  email?: string
  plexUsername?: string
  plexId?: number
  avatar: string
}

export interface AuthenticationUser {
  username: string
  password: string
}

export interface UserWatchHistory {
  user: string
  friendlyName: string
  userId: number
  fullTitle: string
  watchedStatus: number
  ratingKey: number
  parentRatingKey?: number
  grandparentRatingKey?: number
  userThumb?: string
}

export interface MediaInfo {
  images?: Image[]
  releaseDate?: string
  title: string
  ended?: boolean
}

export interface Image {
  coverType?: string
  url?: string
  remoteUrl?: string
}

export interface ServiceInfo {
  host: string
  port?: string
  apiKey: string
  useSsl: boolean
  service: Services
}

export type Services = 'tautulli' | 'overseerr' | 'radarr' | 'sonarr'
