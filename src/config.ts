export type ConsoleType = 'dmg' | 'sgb' | 'mgb' | 'mgl' | 'sgb2' | 'cgb' | 'agb' | 'ags' | 'gbs' | 'oxy'

export const consoles: ConsoleType[] = ['dmg', 'sgb', 'mgb', 'mgl', 'sgb2', 'cgb', 'agb', 'ags', 'gbs', 'oxy'];

export interface ConsoleConfig {
  name: string,
}

export const consoleCfgs: Record<ConsoleType, ConsoleConfig> = {
  'dmg': {name: 'Game Boy'},
  'sgb': {name: 'Super Game Boy'},
  'mgb': {name: 'Game Boy Pocket'},
  'mgl': {name: 'Game Boy Light'},
  'sgb2': {name: 'Super Game Boy 2'},
  'cgb': {name: 'Game Boy Color'},
  'agb': {name: 'Game Boy Advance'},
  'ags': {name: 'Game Boy Advance SP'},
  'gbs': {name: 'Game Boy Player'},
  'oxy': {name: 'Game Boy Micro'},
};
