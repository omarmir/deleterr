export interface Settings {
  newPassword?: String
}
//Delete (season or whole show?) Wait for show to finish? Wait for show to end? If a newer season is in progress
//consider older seasons watched? If a newer season is watched consider older seasons watched?

type SettingOptionCore = { name: string; label: string; value?: string }

export type SettingsOption = {
  name: string
  title: string
  subtitle: string
  additionalDetail?: SettingOptionCore
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
      value: SettingOptionCore[]
    }
)
