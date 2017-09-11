import * as humanDate from 'human-date';

import {Calendar} from '../metadata';

export namespace short {
  export function calendar<T extends Calendar>({year, month, week}: T): string {
    let prefix;
    if (month) {
      prefix = (humanDate.monthName(month).substring(0, 3));
    } else if (week) {
      prefix = String(week)
    }

    const yearStr = (year && String(year)) || '????';
    return (prefix) ? `${prefix}/${yearStr}` : yearStr;
  }
}

export function calendar<T extends Calendar>({year, month, week}: T): string {
  let prefix;
  if (month) {
    prefix = humanDate.monthName(month);
  } else if (week) {
    prefix = `Week ${week}`;
  }

  const yearStr = (year && String(year)) || '????';
  return (prefix) ? `${prefix}/${yearStr}` : yearStr;
}

export function optional<T>(f: (value: T) => string, value: T | null | undefined): string {
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
    case 'bsi':
      return 'BSI';
    case 'fujitsu':
      return 'Fujitsu';
    case 'hynix':
      return 'Hynix';
    case 'microchip':
      return 'Microchip';
    case 'mitsumi':
      return 'Mitsumi';
    case 'mosel-vitelic':
      return 'Mosel-Vitelic'
    case 'nec':
      return 'NEC';
    case 'rohm':
      return 'ROHM';
    case 'sharp':
      return 'Sharp';
    case 'tdk':
      return 'TDK';
    case 'xlink':
      return 'Xlink (?)'
    default:
      return value;
  }
}