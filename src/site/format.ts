import { monthName } from 'human-date'

import { Calendar } from '../metadata'

function shortMonth(month: number): string {
  return monthName(month).substring(0, 3)
}

export namespace short {
  export function calendar<T extends Calendar>({ year, month, week }: T): string {
    let prefix
    if (month) {
      prefix = shortMonth(month)
    } else if (week) {
      prefix = String(week)
    }

    const yearStr = (year && String(year)) || ''
    return prefix ? `${prefix}/${yearStr}` : yearStr
  }
}

export function calendar<T extends Calendar>({ year, month, week }: T): string {
  let prefix
  if (month) {
    prefix = monthName(month)
  } else if (week) {
    prefix = `Week ${week}`
  }

  const yearStr = (year && String(year)) || ''
  return prefix ? `${prefix}/${yearStr}` : yearStr
}
