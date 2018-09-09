import * as R from 'ramda'

export function rejectNil<T>(array: (T | null | undefined)[]): T[] {
  return R.reject(R.isNil, array) as T[]
}
