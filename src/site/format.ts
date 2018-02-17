import {monthName} from 'human-date';

import {Calendar} from '../metadata';

function shortMonth(month: number): string {
  return monthName(month).substring(0, 3)
}

export namespace short {
  export function calendar<T extends Calendar>({year, month, week, date_range}: T): string {
    let prefix;
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

    const yearStr = (year && String(year)) || '????';
    return (prefix) ? `${prefix}/${yearStr}` : yearStr;
  }
}

export function calendar<T extends Calendar>({year, month, week, date_range}: T): string {
  let prefix;
  if (month) {
    prefix = monthName(month);
  } else if (week) {
    prefix = `Week ${week}`;
  } else if (date_range) {
    const [start, end] = date_range
    if (start.month && end.month) {
      prefix = `${monthName(start.month)}-${monthName(end.month)}`
    }
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
    case 'amic':
      return 'AMIC Technology';
    case 'bsi':
      return 'BSI';
    case 'fujitsu':
      return 'Fujitsu';
    case 'hynix':
      return 'Hynix';
    case 'hyundai':
      return 'Hyundai';
    case 'kds':
      return 'Daishinku';
    case 'kss':
      return 'Kinseki';
    case 'lsi-logic':
      return 'LSI Logic';
    case 'macronix':
      return 'Macronix';
    case 'microchip':
      return 'Microchip';
    case 'mitsumi':
      return 'Mitsumi';
    case 'mosel-vitelic':
      return 'Mosel-Vitelic';
    case 'nec':
      return 'NEC';
    case 'rohm':
      return 'ROHM';
    case 'samsung':
      return 'Samsung';
    case 'sanyo':
      return 'Sanyo';
    case 'sharp':
      return 'Sharp';
    case 'st':
      return 'STMicroelectronics';
    case 'tdk':
      return 'TDK';
    case 'texas-instruments':
      return 'Texas Instruments';
    case 'toshiba':
      return 'Toshiba';
    case 'winbond':
      return 'Winbond';
    case 'xlink':
      return 'Xlink (?)';
    default:
      return value;
  }
}