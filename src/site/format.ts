import * as humanDate from 'human-date';

export function formatShortYearWeek<T extends {year?: number, week?: number}>({year, week}: T): string {
  return `${week || '??'}/${year || '????'}`
}
export function formatYearWeek<T extends {year?: number, week?: number}>({year, week}: T): string {
  return `Week ${week || '??'}/${year || '????'}`
}
export function formatShortYearMonth<T extends {year?: number, month?: number}>({year, month}: T): string {
  const monthName = (month && humanDate.monthName(month).substring(0, 3)) || '??';
  return `${monthName}/${year || '????'}`;
}
export function formatYearMonth<T extends {year?: number, month?: number}>({year, month}: T): string {
  const monthName = (month && humanDate.monthName(month)) || '??';
  return `${monthName}/${year || '????'}`;
}
