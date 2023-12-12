export interface Settings {
  newPassword?: String
}
//Delete (season or whole show?) Wait for show to finish? Wait for show to end? If a newer season is in progress
//consider older seasons watched? If a newer season is watched consider older seasons watched?
export interface SettingsOption {
  name: string
  title: string
  value: boolean
  subtitle: string
  additionalDetail?: { name: string, label: string, value?: string }
}
