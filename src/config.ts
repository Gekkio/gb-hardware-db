import * as fs from 'fs-extra'

export type ConsoleType = 'dmg' | 'sgb' | 'mgb' | 'mgl' | 'sgb2' | 'cgb' | 'agb' | 'ags' | 'gbs' | 'oxy'

export const consoles: ConsoleType[] = ['dmg', 'sgb', 'mgb', 'mgl', 'sgb2', 'cgb', 'agb', 'ags', 'gbs', 'oxy']

export interface ConsoleConfig {
  name: string
}

export const consoleCfgs: Record<ConsoleType, ConsoleConfig> = {
  dmg: { name: 'Game Boy' },
  sgb: { name: 'Super Game Boy' },
  mgb: { name: 'Game Boy Pocket' },
  mgl: { name: 'Game Boy Light' },
  sgb2: { name: 'Super Game Boy 2' },
  cgb: { name: 'Game Boy Color' },
  agb: { name: 'Game Boy Advance' },
  ags: { name: 'Game Boy Advance SP' },
  gbs: { name: 'Game Boy Player' },
  oxy: { name: 'Game Boy Micro' },
}

export interface CartLayout {
  chips: CartChip[]
}

export interface CartChip {
  designator: string
  name: string
  key:
    | 'rom'
    | 'mapper'
    | 'ram'
    | 'ram_protector'
    | 'u4'
    | 'u5'
    | 'line_decoder'
    | 'flash'
    | 'accelerometer'
    | 'eeprom'
    | 'rom2'
    | 'crystal'
}

export type CartLayoutId =
  | 'rom'
  | 'rom_mapper'
  | 'rom_mapper_ram'
  | 'rom_mapper_ram_xtal'
  | 'mbc2'
  | 'mbc6'
  | 'mbc7'
  | 'type_15'
  | 'huc3'
  | 'tama'

export const gameLayouts: Record<CartLayoutId, CartLayout> = {
  rom: {
    chips: [{ designator: 'U1', name: 'ROM', key: 'rom' }],
  },
  rom_mapper: {
    chips: [{ designator: 'U1', name: 'ROM', key: 'rom' }, { designator: 'U2', name: 'Mapper', key: 'mapper' }],
  },
  rom_mapper_ram: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'RAM', key: 'ram' },
      { designator: 'U4', name: 'RAM protector', key: 'ram_protector' },
    ],
  },
  rom_mapper_ram_xtal: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'RAM', key: 'ram' },
      { designator: 'U4', name: 'RAM protector', key: 'ram_protector' },
      { designator: 'X1', name: 'Crystal', key: 'crystal' },
    ],
  },
  mbc2: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'RAM protector', key: 'ram_protector' },
    ],
  },
  mbc6: {
    chips: [
      { designator: 'U1', name: 'Mapper', key: 'mapper' },
      { designator: 'U2', name: 'ROM', key: 'rom' },
      { designator: 'U3', name: 'Flash', key: 'flash' },
      { designator: 'U4', name: 'RAM', key: 'ram' },
      { designator: 'U5', name: 'RAM protector', key: 'ram_protector' },
    ],
  },
  mbc7: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'EEPROM', key: 'eeprom' },
      { designator: 'U4', name: 'Accelerometer', key: 'accelerometer' },
    ],
  },
  type_15: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'RAM', key: 'ram' },
      { designator: 'U4', name: 'RAM protector', key: 'ram_protector' },
      { designator: 'U5', name: 'ROM 2', key: 'rom2' },
      { designator: 'U6', name: 'Line Decoder', key: 'line_decoder' },
    ],
  },
  huc3: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'RAM', key: 'ram' },
      { designator: 'U4', name: 'RAM protector', key: 'ram_protector' },
      { designator: 'U5', name: '????', key: 'u5' },
      { designator: 'X1', name: 'Crystal', key: 'crystal' },
    ],
  },
  tama: {
    chips: [
      { designator: 'U1', name: 'ROM', key: 'rom' },
      { designator: 'U2', name: 'Mapper', key: 'mapper' },
      { designator: 'U3', name: 'MCU', key: 'ram' },
      { designator: 'U4', name: 'RTC', key: 'u4' },
      { designator: 'U5', name: 'RAM protector', key: 'ram_protector' },
      { designator: 'X1', name: 'Crystal', key: 'crystal' },
    ],
  },
}

export interface MapperConfig {
  name: string
}

export type MapperId =
  | 'no-mapper'
  | 'mbc1'
  | 'mbc2'
  | 'mbc3'
  | 'mbc30'
  | 'mbc5'
  | 'mbc6'
  | 'mbc7'
  | 'mmm01'
  | 'huc1'
  | 'huc3'
  | 'tama5'

export const mapperCfgs: Record<MapperId, MapperConfig> = {
  'no-mapper': { name: 'No mapper' },
  mbc1: { name: 'MBC1' },
  mbc2: { name: 'MBC2' },
  mbc3: { name: 'MBC3' },
  mbc30: { name: 'MBC30' },
  mbc5: { name: 'MBC5' },
  mbc6: { name: 'MBC6' },
  mbc7: { name: 'MBC7' },
  mmm01: { name: 'MMM01' },
  huc1: { name: 'HuC-1' },
  huc3: { name: 'HuC-3' },
  tama5: { name: 'TAMA5' },
}

export interface GameConfig {
  name: string
  platform: 'gb' | 'gbc'
  layouts: CartLayoutId[]
}

export const gameCfgs: Record<string, GameConfig> = fs.readJsonSync('config/games.json')
