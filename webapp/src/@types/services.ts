import { APIStatus, Services } from './deleterr'

export interface ServiceStatus {
  service: Services
  isSuccess: boolean
  status: APIStatus
}

export type AllServiceStatus = { [key in Services]: ServiceStatus }
