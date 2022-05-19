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
