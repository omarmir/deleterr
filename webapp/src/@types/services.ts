import { OperationState } from './common'
import { Services } from './deleterr'

export interface ServiceStatus {
  test: OperationState
  save: OperationState
  errorMsg: string
}

export enum ServiceOperations {
  Save = 'save',
  Test = 'test',
}

export type AllServiceStatus = { [key in Services]: ServiceStatus }
