import { monthName } from 'human-date'

import { Calendar } from '../metadata'

function shortMonth(month: number): string {
  return monthName(month).substring(0, 3)
}

export namespace short {
  export function calendar<T extends Calendar>({ year, month, week, date_range }: T): string {
    let prefix
    if (month) {
      prefix = shortMonth(month)
    } else if (week) {
      prefix = String(week)
    } else if (date_range) {
      const [start, end] = date_range
      if (start.month && end.month) {
        prefix = `${shortMonth(start.month)}-${shortMonth(end.month)}`
      }
    }

    const yearStr = (year && String(year)) || '????'
    return prefix ? `${prefix}/${yearStr}` : yearStr
  }
}

export function calendar<T extends Calendar>({ year, month, week, date_range }: T): string {
  let prefix
  if (month) {
    prefix = monthName(month)
  } else if (week) {
    prefix = `Week ${week}`
  } else if (date_range) {
    const [start, end] = date_range
    if (start.month && end.month) {
      prefix = `${monthName(start.month)}-${monthName(end.month)}`
    }
  }

  const yearStr = (year && String(year)) || '????'
  return prefix ? `${prefix}/${yearStr}` : yearStr
}

export function optional<T>(f: (value: T) => string, value: T | null | undefined): string {
  if (value === undefined) {
    return '????'
  } else if (value === null) {
    return '-'
  } else {
    return f(value)
  }
}

export function manufacturer(value: string): string {
  switch (value) {
    case 'amic':
      return 'AMIC Technology'
    case 'hynix':
      return 'Hynix'
    case 'kss':
      return 'Kinseki'
    case 'microchip':
      return 'Microchip'
    case 'st':
      return 'STMicroelectronics'
    case 'tdk':
      return 'TDK'
    default:
      return value
  }
}
