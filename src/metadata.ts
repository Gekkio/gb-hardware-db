import * as Joi from 'joi'

export interface Calendar {
  year?: number
  month?: number
  week?: number
  date_range?: [DateRangePart, DateRangePart]
}

const schemas = {
  year: Joi.number()
    .integer()
    .min(1988)
    .max(2010),
  month: Joi.number()
    .integer()
    .min(1)
    .max(12),
  week: Joi.number()
    .integer()
    .min(1)
    .max(53),
}

export interface DateRangePart {
  month?: number
  part?: number
}

export interface Chip {
  type?: string
  label?: string | null
  manufacturer?: string
  year?: number
  month?: number
  week?: number
  outlier?: boolean
}

export interface DmgMetadata {
  type: 'DMG'
  color?: string
  screws?: string
  year?: number
  month?: number
  mainboard: {
    type: string
    circled_letters?: string | null
    extra_label?: string
    stamp?: string
    cpu?: Chip
    work_ram?: Chip
    video_ram?: Chip
    amplifier?: Chip
    crystal?: Chip
  }
  lcd_board?: {
    type: string
    circled_letters?: string
    stamp?: string
    year?: number
    month?: number
    lcd_panel?: {
      label: string
      year?: number
      month?: number
    }
    column_driver?: Chip
    row_driver?: Chip
    regulator?: Chip
  }
  power_board?: {
    type: string
    label: string
    year?: number
    month?: number
  }
  jack_board?: {
    type: string
    extra_label?: string | null
  }
}

export interface SgbMetadata {
  type: 'SGB'
  stamp?: string
  mainboard: {
    type: string
    circled_letters?: string
    letter_at_top_right?: string
    cpu?: Chip
    icd2?: Chip
    work_ram?: Chip
    video_ram?: Chip
    rom?: Chip
    cic?: Chip
    year?: number
    month?: number
  }
}

export interface MgbMetadata {
  type: 'MGB'
  color?: string
  release_code?: string
  year?: number
  month?: number
  mainboard: {
    type: string
    circled_letters?: string | null
    number_pair?: string
    stamp?: string
    year?: number
    month?: number
    date_range?: [DateRangePart, DateRangePart]
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
  }
  lcd?: {
    column_driver?: Chip
    row_driver?: Chip
  }
}

export interface MglMetadata {
  type: 'MGL'
  color?: string
  release_code?: string
  year?: number
  week?: number
  mainboard: {
    type: string
    circled_letters?: string | null
    number_pair?: string
    stamp?: string
    year?: number
    month?: number
    date_range?: [DateRangePart, DateRangePart]
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
    t1?: Chip
  }
  lcd?: {
    column_driver?: Chip
    row_driver?: Chip
  }
}

export interface Sgb2Metadata {
  type: 'SGB2'
  stamp?: string
  mainboard: {
    type: string
    circled_letters?: string
    letter_at_top_right?: string
    crystal?: Chip
    cpu?: Chip
    icd2?: Chip
    work_ram?: Chip
    rom?: Chip
    cic?: Chip
    coil?: Chip
    year?: number
    month?: number
  }
}

export interface CgbMetadata {
  type: 'CGB'
  color?: string
  release_code?: string
  year?: number
  month?: number
  week?: number
  mainboard: {
    type: string
    circled_letters?: string | null
    number_pair?: string
    stamp?: string
    year?: number
    month?: number
    date_range?: [DateRangePart, DateRangePart]
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
  }
}

export interface AgbMetadata {
  type: 'AGB'
  color?: string
  release_code?: string
  year?: number
  week?: number
  mainboard: {
    type: string
    number_pair?: string
    stamp?: string
    circled_letters?: string
    crystal?: Chip
    cpu?: Chip
    work_ram?: Chip
    regulator?: Chip | null
    amplifier?: Chip
    u4?: Chip
    year?: number
    month?: number
  }
}

export interface AgsMetadata {
  type: 'AGS'
  color?: string
  mainboard: {
    type: string
    number_pair?: string
    stamp?: string
    circled_letters?: string
    crystal?: Chip
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    u4?: Chip
    u5?: Chip
    year?: number
    month?: number
  }
}

export interface GbsMetadata {
  type: 'GBS'
  color?: string
  release_code?: string
  year?: number
  week?: number
  mainboard: {
    type: string
    number_pair?: string
    stamp?: string
    stamp_front?: string
    stamp_back?: string
    circled_letters?: string
    year?: number
    month?: number
    crystal?: Chip
    cpu?: Chip
    work_ram?: Chip
    u4?: Chip
    u5?: Chip
    u6?: Chip
  }
}

export interface OxyMetadata {
  type: 'OXY'
  color?: string
  release_code?: string
  mainboard: {
    type: string
    circled_letters?: string
    cpu?: Chip
    u2?: Chip
    u4?: Chip
    u5?: Chip
    year?: number
    month?: number
  }
}

export interface CartridgeMetadata {
  code?: string
  stamp?: string
  board: {
    type: string
    circled_letters?: string
    extra_label?: string
    year?: number
    month?: number
    rom?: Chip
    rom2?: Chip
    mapper?: Chip
    ram?: Chip
    ram_protector?: Chip
    flash?: Chip
    u4?: Chip
    u5?: Chip
    line_decoder?: Chip
    eeprom?: Chip
    accelerometer?: Chip
    crystal?: Chip
  }
}
