declare module 'human-date' {
  interface PrettyPrintOptions {
    showTime?: boolean
  }
  export function prettyPrint(input?: Date | string, options?: PrettyPrintOptions): string
  export function prettyPrint(seconds?: number, options?: PrettyPrintOptions): string

  interface RelativeTimeOptions {
    futureSuffix?: string
    pastSuffx?: string
    presentText?: string
    // returnObject?: boolean;
    allUnits?: boolean
  }
  export function relativeTime(input?: Date | string, options?: RelativeTimeOptions): string
  export function relativeTime(seconds?: number, options?: RelativeTimeOptions): string

  export function monthName(index: number): string
  export function monthName(input: Date | string): string

  export function toUTC(input?: Date | string): Date
  export function toUTC(epochtime?: number): Date
}
