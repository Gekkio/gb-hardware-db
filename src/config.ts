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
  key: 'rom' | 'mapper' | 'ram' | 'ram_protector' | 'u4' | 'u5' | 'line_decoder' | 'flash' | 'accelerometer' | 'eeprom' | 'rom2'
}

export type CartLayoutId = 'rom' | 'rom_mbc' | 'rom_mbc_ram' | 'rom_mbc_protect' | 'rom_mbc_ram_xtal' | 'huc3' | 'tama' | 'mbc6' | 'mbc7' | 'a15'

export const gameLayouts: Record<CartLayoutId, CartLayout> = {
  'rom': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
    ],
  },
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
  'rom_mbc_protect': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM protector', key: 'ram_protector'},
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
  },
  'huc3': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM', key: 'ram'},
      {designator: 'U4', name: 'RAM protector', key: 'ram_protector'},
      {designator: 'U5', name: '????', key: 'u5'},
    ],
    crystal: 'X1',
    battery: true,
  },
  'tama': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM', key: 'ram'},
      {designator: 'U4', name: '????', key: 'u4'},
      {designator: 'U5', name: 'RAM protector', key: 'ram_protector'},
    ],
    crystal: 'X1',
    battery: true,
  },
  'mbc6': {
    chips: [
      {designator: 'U1', name: 'Mapper', key: 'mapper'},
      {designator: 'U2', name: 'ROM', key: 'rom'},
      {designator: 'U4', name: 'Flash', key: 'flash'},
      {designator: 'U4', name: 'RAM', key: 'ram'},
      {designator: 'U5', name: 'RAM protector', key: 'ram_protector'},
    ],
    battery: true,
  },
  'mbc7': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'EEPROM', key: 'eeprom'},
      {designator: 'U4', name: 'Accelerometer', key: 'accelerometer'},
    ],
  },
  'a15': {
    chips: [
      {designator: 'U1', name: 'ROM', key: 'rom'},
      {designator: 'U2', name: 'Mapper', key: 'mapper'},
      {designator: 'U3', name: 'RAM', key: 'ram'},
      {designator: 'U4', name: 'RAM protector', key: 'ram_protector'},
      {designator: 'U5', name: 'ROM 2', key: 'rom2'},
      {designator: 'U6', name: 'Line Decoder', key: 'line_decoder'},
    ],
    battery: true,
  }
};

export interface MapperConfig {
  name: string,
}

export type MapperId = 'no-mapper' | 'mbc1' | 'mbc2' | 'mbc3' | 'mbc30' | 'mbc5' | 'mbc6' | 'mbc7' | 'mmm01' | 'huc1' | 'huc3' | 'tama5'

export const mapperCfgs: Record<MapperId, MapperConfig> = {
  'no-mapper': {name: 'No mapper'},
  'mbc1': {name: 'MBC1'},
  'mbc2': {name: 'MBC2'},
  'mbc3': {name: 'MBC3'},
  'mbc30': {name: 'MBC30'},
  'mbc5': {name: 'MBC5'},
  'mbc6': {name: 'MBC6'},
  'mbc7': {name: 'MBC7'},
  'mmm01': {name: 'MMM01'},
  'huc1': {name: 'HuC-1'},
  'huc3': {name: 'HuC-3'},
  'tama5': {name: 'TAMA5'},
};

export interface GameConfig {
  name: string,
  layout: CartLayoutId,
}

export const gameCfgs: Record<string, GameConfig> = {
  'CGB-AW8A-0': {name: 'Wario Land 3 (World) (En,Ja)', layout: 'rom_mbc_ram'},
  'CGB-AZ7J-0': {name: 'Zelda no Densetsu - Fushigi no Kimi - Daichi no Shou (Japan)', layout: 'rom_mbc_ram'},
  'CGB-AZ7P-0': {name: 'Legend of Zelda, The - Oracle of Seasons (Europe) (En,Fr,De,Es,It)', layout: 'rom_mbc_ram'},
  'CGB-AZ8P-0': {name: 'Legend of Zelda, The - Oracle of Ages (Europe) (En,Fr,De,Es,It)', layout: 'rom_mbc_ram'},
  'CGB-B82J-0': {name: 'Densha de Go! 2 (Japan)', layout: 'a15'},
  'CGB-B9AJ-0': {name: 'Mobile Trainer (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BDSP-0': {name: 'Dinosaur\'us (Europe) (En,Fr,De,Es,It,Nl)', layout: 'rom_mbc_ram'},
  'CGB-BFUP-0': {name: 'Shrek - Fairy Tale Freakdown (USA, Europe) (En,Fr,De,Es,It)', layout: 'rom_mbc'},
  'CGB-BFVJ-1': {name: 'Monster Traveler (Japan) (Rev A)', layout: 'rom_mbc_ram'},
  'CGB-BGLJ-0': {name: 'Super Gals! Kotobuki Ran (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BHMJ-0': {name: 'Hamster Paradise 2 (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BHVE-0': {name: 'Harry Potter and the Sorcerer\'s Stone (USA, Europe) (En,Fr,De,Es,It,Nl,Pt,Sv,No,Da,Fi)', layout: 'rom_mbc_ram'},
  'CGB-BMVJ-0': {name: 'Net de Get - Minigame @ 100 (Japan)', layout: 'mbc6'},
  'CGB-BXTJ-0': {name: 'Pocket Monsters - Crystal Version (Japan)', layout: 'rom_mbc_ram_xtal'},
  'CGB-BY3J-0': {name: 'Yu-Gi-Oh! Duel Monsters III - Tri Holy God Advant (Japan)', layout: 'rom_mbc_ram'},
  'CGB-BY4J-0': {name: 'Yu-Gi-Oh! Duel Monsters 4 - Battle of Great Duelist - Yuugi Deck (Japan)', layout: 'rom_mbc_ram'},
  'CGB-HF2J-0': {name: 'Pocket Family GB2 (Japan)', layout: 'huc3'},
  'CGB-KCEJ-0': {name: 'Command Master (Japan)', layout: 'mbc7'},
  'CGB-KKKJ-0': {name: 'Korokoro Kirby (Japan)', layout: 'mbc7'},
  'DMG-A4RJ-0': {name: 'Bakukyuu Renpatsu!! Super B-Daman - Gekitan! Rising Valkyrie!! (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AAUJ-1': {name: 'Pocket Monsters Kin (Japan) (Rev A) (SGB Enhanced)', layout: 'rom_mbc_ram_xtal'},
  'DMG-ABUP-0': {name: 'Bust-A-Move 2 - Arcade Edition (USA, Europe)', layout: 'rom_mbc'},
  'DMG-ACXJ-0': {name: 'Pokemon Card GB (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AD3P-1': {name: 'Donkey Kong Land III (USA, Europe) (Rev A) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AD4P-0': {name: 'Dropzone (Europe)', layout: 'rom_mbc'},
  'DMG-ADDJ-0': {name: 'Donkey Kong Land (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-ADDP-0': {name: 'Donkey Kong Land 2 (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-ADQJ-0': {name: 'Dragon Quest Monsters - Terry no Wonderland (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AFFJ-0': {name: 'Super Chinese Fighter GB (Japan) (SGB Enhanced)', layout: 'rom_mbc'},
  'DMG-AFGE-0': {name: 'Frogger (USA)', layout: 'rom_mbc'},
  'DMG-AGOP-0': {name: 'Hugo (Europe) (SGB Enhanced)', layout: 'rom_mbc'},
  'DMG-AGQE-0': {name: 'Game & Watch Gallery 3 (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AK2P-0': {name: 'Mortal Kombat & Mortal Kombat II (USA, Europe)', layout: 'rom_mbc'},
  'DMG-AKMJ-0': {name: 'Kandume Monsters (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram_xtal'},
  'DMG-AM3J-0': {name: 'Momotarou Collection 2 (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AM6J-0': {name: 'Daikaijuu Monogatari - The Miracle of the Zone II (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AMDJ-0': {name: 'Momotarou Collection (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-ANWJ-0': {name: 'Itsudemo! Nyan to Wonderful (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram_xtal'},
  'DMG-AODP-0': {name: 'Oddworld Adventures (USA, Europe)', layout: 'rom_mbc'},
  'DMG-AOMJ-0': {name: 'Game de Hakken!! Tamagotchi - Osutchi to Mesutchi (Japan) (SGB Enhanced)', layout: 'tama'},
  'DMG-APDP-0': {name: 'Pinball Deluxe (Europe)', layout: 'rom_mbc'},
  'DMG-APEE-0': {name: 'Pokemon - Blue Version (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram_xtal'},
  'DMG-APOJ-0': {name: 'Pocket Bomber Man (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-APSE-0': {name: 'Pokemon - Yellow Version - Special Pikachu Edition (USA, Europe) (GBC,SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-ATAJ-0': {name: 'Game de Hakken!! Tamagotchi (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AVBJ-0': {name: 'Pocket Bowling (Japan)', layout: 'rom_mbc'},
  'DMG-AW2P-0': {name: 'Wario Land II (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AW7E-0': {name: 'James Bond 007 (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AWDJ-0': {name: 'Dino Breeder (Japan) (SGB Enhanced)', layout: 'rom_mbc_protect'},
  'DMG-AWLP-0': {name: 'Wario Land II (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-AYJ-0': {name: 'Ayakashi no Shiro (Japan)', layout: 'rom_mbc_protect'},
  'DMG-AYMJ-1': {name: 'Gakkyuu Ou Yamazaki (Japan) (Rev A)', layout: 'rom_mbc_ram'},
  'DMG-AYWJ-0': {name: 'Bokujou Monogatari GB (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram_xtal'},
  'DMG-AZLP-0': {name: 'Legend of Zelda, The - Link\'s Awakening DX (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-B7HJ-0': {name: 'Nakayoshi Pet Series 1 - Kawaii Hamster (Japan)', layout: 'rom_mbc_ram'},
  'DMG-BLUJ-0': {name: 'From TV Animation One Piece - Yume no Luffy Kaizokudan Tanjou! (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-BMAP-0': {name: 'Mary-Kate and Ashley - Pocket Planner (USA, Europe)', layout: 'rom_mbc_ram_xtal'},
  'DMG-BQLJ-0': {name: 'Dragon Quest Monsters 2 - Maruta no Fushigi na Kagi - Ruka no Tabidachi (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-CNE-0': {name: 'Operation C (USA)', layout: 'rom_mbc'},
  'DMG-CVJ-0': {name: 'Dracula Densetsu (Japan)', layout: 'rom_mbc'},
  'DMG-DDE-0': {name: 'Double Dragon (USA, Europe)', layout: 'rom_mbc'},
  'DMG-F1A-1': {name: 'F-1 Race (World) (Rev A)', layout: 'rom_mbc_protect'},
  'DMG-G2E-0': {name: 'Gauntlet II (USA, Europe)', layout: 'rom_mbc'},
  'DMG-HBA-0': {name: 'Bomber Boy (Japan)', layout: 'rom_mbc'},
  'DMG-HFAJ-0': {name: 'Pocket Family (Japan) (SGB Enhanced)', layout: 'huc3'},
  'DMG-HFE-0': {name: 'Hunt for Red October, The (USA, Europe)', layout: 'rom_mbc'},
  'DMG-HLA-1': {name: 'Hyper Lode Runner (World) (Rev A)', layout: 'rom'},
  'DMG-HQE-0': {name: 'Chase H.Q. (USA, Europe)', layout: 'rom_mbc'},
  'DMG-HRCJ-0': {name: 'Robot Poncots - Star Version (Japan) (SGB Enhanced)', layout: 'huc3'},
  'DMG-L6J-2': {name: 'Super Mario Land 2 - 6-tsu no Kinka (Japan) (Rev 2)', layout: 'rom_mbc_ram'},
  'DMG-LDE-0': {name: 'Little Mermaid, The (USA)', layout: 'rom_mbc'},
  'DMG-LWE-0': {name: 'WWF Superstars (USA, Europe)', layout: 'rom_mbc'},
  'DMG-MLA-1': {name: 'Super Mario Land (World) (Rev A)', layout: 'rom_mbc'},
  'DMG-NMX-0': {name: 'Nemesis (Europe)', layout: 'rom_mbc'},
  'DMG-PBJ-0': {name: 'Pinball - 66hiki no Wani Daikoushin! (Japan)', layout: 'rom_mbc'},
  'DMG-PCE-0': {name: 'Pac-Man (USA)', layout: 'rom_mbc'},
  'DMG-Q6E-0': {name: 'NFL Quarterback Club (USA, Europe)', layout: 'rom_mbc'},
  'DMG-REA-0': {name: 'R-Type (Japan)', layout: 'rom_mbc'},
  'DMG-RWE-0': {name: 'Mega Man - Dr. Wily\'s Revenge (USA)', layout: 'rom_mbc'},
  'DMG-SAJ-0': {name: 'Makai Toushi Sa-Ga (Japan)', layout: 'rom_mbc_protect'},
  'DMG-SVJ-0': {name: 'Seaside Volley (Japan)', layout: 'rom_mbc'},
  'DMG-TRA-1': {name: 'Tetris (World) (Rev A)', layout: 'rom'},
  'DMG-UHE-0': {name: 'Prehistorik Man (USA, Europe)', layout: 'rom_mbc'},
  'DMG-VPHE-0': {name: 'Pokemon Pinball (USA) (Rumble Version) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-VPHP-0': {name: 'Pokemon Pinball (Europe) (En,Fr,De,Es,It) (Rumble Version) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-VUA-1': {name: 'Dr. Mario (World) (Rev A)', layout: 'rom'},
  'DMG-W6J-0': {name: 'Wizardry Gaiden 3 - Yami no Seiten (Japan)', layout: 'rom_mbc_ram'},
  'DMG-WJA-0': {name: 'Wario Land - Super Mario Land 3 (World)', layout: 'rom_mbc_ram'},
  'DMG-WWE-0': {name: 'Wizards & Warriors Chapter X - The Fortress of Fear (USA, Europe)', layout: 'rom_mbc'},
  'DMG-XCA-0': {name: 'Power Mission (Japan)', layout: 'rom_mbc'},
  'DMG-XTX-0': {name: 'Top Ranking Tennis (Europe)', layout: 'rom_mbc_protect'},
  'DMG-YTE-0': {name: 'Donkey Kong Land (USA, Europe) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-YTJ-0': {name: 'Super Donkey Kong GB (Japan) (SGB Enhanced)', layout: 'rom_mbc_ram'},
  'DMG-ZLE-0': {name: 'Legend of Zelda, The - Link\'s Awakening (USA, Europe)', layout: 'rom_mbc_ram'},
};

export const games: string[] = Object.keys(gameCfgs);
