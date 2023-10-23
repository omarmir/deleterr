export interface APIResponse<T> {
    code: number,
    success: boolean,
    data: T
}
export interface MediaRequest {
    id: number
    status: number
    requestedBy: User
    media: Media
}

export interface Media {
    id: number
    mediaType?: MediaType
    tmdbId?: number
    tvdbId?: number
    ratingKey?: bigint
    status: number
}

export type MediaType = 'movie' | 'tv'

export interface RequestStatus {
    mediaRequest: MediaRequest
    userWatchHistory?: UserWatchHistory
}

export interface User {
    id: number
    username?: string
    userType: number
    email?: string
    plexUsername?: string
    plexId?: bigint
}

export interface UserWatchHistory {
    user: string
    friendlyName: string
    userId: bigint
    fullTitle: string
    watchedStatus: number
    ratingKey: bigint
    parentRatingKey?: bigint
    grandparentRatingKey?: bigint
}
