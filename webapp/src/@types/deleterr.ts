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
  userWatchHistory?: UserWatchHistory
  mediaInfo?: MediaInfo
}

export interface RequestStatusWithRecordInfo {
  allRequests: number
  filteredRequests: number
  requests: RequestStatus[]
}

export type MediaExemption = { [key: string]: number }

export type SingleMediaExeption = [requestId: number, tmdbId: number]

export type DeleteMedia = [requestId: number, id: number]

export enum APIStatus {
  Success,
  WrongAPIKey,
  Other,
  NotFound,
}

export interface RadarrDeleteResponse {
  status: APIStatus
  isSuccess: boolean
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
  posterPath?: string
  releaseDate?: string
  title: string
}

export enum InputType {
  text = 'text',
  number = 'number',
  password = 'password',
}

export enum TestState {
  loading,
  success,
  failure,
  hidden,
}

export interface ServiceInfo {
  host: string
  port?: string
  apiKey: string
  useSsl: boolean
  service: Services
}

export type Services = 'tautulli' | 'overseerr' | 'radarr' | 'sonarr'
