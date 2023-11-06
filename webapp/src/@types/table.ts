type Sortables = 'name' | 'requestedDate' | 'mediaType' | 'watched' | 'user'

interface TableState {
  sortBy?: Sortables
  isDescending?: boolean
  take: number
  skip?: number
  search?: string
}
