import {CartridgeMetadata} from './metadata';

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

export interface CartLayout {
  chips: CartChip[],
  crystal?: string,
  battery?: boolean,
}

export interface CartChip {
  designator: string,
  name: string,
  key: 'rom' | 'mapper' | 'ram' | 'ram_protector'
}

export type CartLayoutId = 'rom_mbc' | 'rom_mbc_ram' | 'rom_mbc_ram_xtal'

export const gameLayouts: Record<CartLayoutId, CartLayout> = {
  'rom_mbc': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
    ],
  },
  'rom_mbc_ram': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM', key: 'ram'},
      {designator: 'U4', name: 'RAM protector', key: 'ram_protector'},
    ],
    battery: true,
  },
  'rom_mbc_ram_xtal': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM', key: 'ram'},
      {designator: 'U4', name: 'RAM protector', key: 'ram_protector'},
    ],
    crystal: 'X1',
    battery: true,
  }
};

export interface GameConfig {
  name: string,
  layout: CartLayoutId,
}

export const gameCfgs: Record<string, GameConfig> = {
  'CGB-AZ7J-0': {name: 'Zelda no Densetsu - Fushigi no Kinomi - Daichi no Shou (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BFUP-0': {name: 'Shrek - Fairy Tale Freakdown (USA, Europe) (En,Fr,De,Es,It)', layout: 'rom_mbc'},
  'CGB-BHMJ-0': {name: 'Hamster Paradise 2 (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BXTJ-0': {name: 'Pocket Monsters - Crystal Version (Japan)', layout: 'rom_mbc_ram_xtal'},
  'CGB-BY3J-0': {name: 'Yu-Gi-Oh! Duel Monsters III - Tri Holy God Advant (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BY4J-0': {name: 'Yu-Gi-Oh! Duel Monsters 4 - Battle of Great Duelist - Yuugi Deck (Japan)', layout: 'rom_mbc_ram'},
  /*
  'DMG-A4RJ-0': {name: 'Bakukyuu Renpatsu!! Super B-Daman - Gekitan! Rising Valkyrie!! (Japan) (SGB Enhanced)'},
  'DMG-AAUJ-1': {name: 'Pocket Monsters Kin (Japan) (Rev A) (SGB Enhanced)'},
  'DMG-ABUP-0': {name: 'Bust-A-Move 2 - Arcade Edition (USA, Europe)'},
  'DMG-AD3P-1': {name: 'Donkey Kong Land III (USA, Europe) (Rev A) (SGB Enhanced)'},
  'DMG-ADDP-0': {name: 'Donkey Kong Land 2 (USA, Europe) (SGB Enhanced)'},
  'DMG-ADQJ-0': {name: 'Dragon Quest Monsters - Terry no Wonderland (Japan) (SGB Enhanced)'},
  'DMG-AFGE-0': {name: 'Frogger (USA)'},
  'DMG-AGOP-0': {name: 'Hugo (Europe) (SGB Enhanced)'},
  'DMG-AM3J-0': {name: 'Momotarou Collection 2 (Japan) (SGB Enhanced)'},
  'DMG-AODP-0': {name: 'Oddworld Adventures (USA, Europe)'},
  'DMG-AWDJ-0': {name: 'Dino Breeder (Japan) (SGB Enhanced)'},
  'DMG-B7HJ-0': {name: 'Nakayoshi Pet Series 1 - Kawaii Hamster (Japan)'},
  'DMG-BLUJ-0': {name: 'From TV Animation One Piece - Yume no Luffy Kaizokudan Tanjou! (Japan) (SGB Enhanced)'},
  'DMG-BMAP-0': {name: 'Mary-Kate and Ashley - Pocket Planner (USA, Europe)'},
  'DMG-CNE-0': {name: 'Operation C (USA)'},
  'DMG-CVJ-0': {name: 'Dracula Densetsu (Japan)'},
  'DMG-DDE-0': {name: 'Double Dragon (USA, Europe)'},
  'DMG-F1A-1': {name: 'F-1 Race (World) (Rev A)'},
  'DMG-G2E-0': {name: 'Gauntlet II (USA, Europe)'},
  'DMG-HFE-0': {name: 'Hunt for Red October, The (USA, Europe)'},
  'DMG-HQE-0': {name: 'Chase H.Q. (USA, Europe)'},
  'DMG-L6J-2': {name: 'Super Mario Land 2 - 6-tsu no Kinka (Japan) (Rev 2)'},
  'DMG-LDE-0': {name: 'Little Mermaid, The (USA)'},
  'DMG-MLA-1': {name: 'Super Mario Land (World) (Rev A)'},
  'DMG-NMX-0': {name: 'Nemesis (Europe)'},
  'DMG-PBJ-0': {name: 'Pinball - 66hiki no Wani Daikoushin! (Japan)'},
  'DMG-PCE-0': {name: 'Pac-Man (USA)'},
  'DMG-Q6E-0': {name: 'NFL Quarterback Club (USA, Europe)'},
  'DMG-RWE-0': {name: 'Mega Man - Dr. Wily\'s Revenge (USA)'},
  'DMG-SVJ-0': {name: 'Seaside Volley (Japan)'},
  'DMG-TRA-1': {name: 'Tetris (World) (Rev A)'},
  'DMG-VUA-1': {name: 'Dr. Mario (World) (Rev A)'},
  'DMG-YTE-0': {name: 'Donkey Kong Land (USA, Europe) (SGB Enhanced)'},
  */
};

export const games: string[] = Object.keys(gameCfgs);
