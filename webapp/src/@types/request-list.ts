export type ListQueryParams = {
    sortBy: 'requestedDate'
    isDescending: boolean,
    take: number,
    skip: number,
    search: string | null,
}