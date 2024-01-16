export type SettingNames = 'showEnd' | 'purgeSeason' | 'watchedMarker' | 'tvPurgePeriod' | 'moviePurgePeriod' | 'tvPurgePeriodDays' | 'moviePurgePeriodDays' | 'tautulliWatchedMarker' | 'inProgresswatchedMarker' | 'LatestWatchedwatchedMarker'

export type Settings = Record<SettingNames, string | boolean>

type SettingsAdditionalDetails = { name: SettingNames; label: string; value?: string }


export type SettingsOption = {
  name: SettingNames
  title: string
  subtitle: string
  additionalDetail?: SettingsAdditionalDetails
} & (
    | {
      type: 'boolean'
      value: boolean
    }
    | {
      type: 'string'
      value: string
    }
    | {
      type: 'array'
      value: SettingsAdditionalDetails[]
    }
  )
