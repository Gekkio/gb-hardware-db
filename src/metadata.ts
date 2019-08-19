export interface Calendar {
  year?: number
  month?: number
  week?: number
}

export interface Chip {
  kind?: string
  label?: string | null
  manufacturer?: string
  year?: number
  month?: number
  week?: number
  rom_code?: string
  outlier?: boolean
}

export interface LcdPanel {
  label?: string
  year?: number
  month?: number
  column_driver?: Chip
  row_driver?: Chip
}

export interface DmgMetadata {
  color?: string
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
    lcd_panel?: LcdPanel
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
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
  }
  lcd_panel?: LcdPanel
}

export interface MglMetadata {
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
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
    t1?: Chip
  }
  lcd_panel?: LcdPanel
}

export interface Sgb2Metadata {
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
    cpu?: Chip
    work_ram?: Chip
    amplifier?: Chip
    regulator?: Chip
    crystal?: Chip
  }
}

export interface AgbMetadata {
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
