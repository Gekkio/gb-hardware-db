import * as R from 'ramda';
import * as fs from 'fs-extra';
import * as path from 'path';
import * as Joi from 'joi';
import * as urlSlug from 'url-slug';

import {
  AgbMetadata, AgsMetadata, CgbMetadata, DmgMetadata, GbsMetadata, Metadata, MgbMetadata, MglMetadata, OxyMetadata,
  Sgb2Metadata, SgbMetadata
} from './metadata';
import {rejectNil} from './util/arrays'

export interface FsEntry {
  absolutePath: string;
  name: string;
  stats: fs.Stats;
}

async function fsEntry(basePath: string, name: string): Promise<FsEntry> {
  const absolutePath = path.resolve(basePath, name);
  const stats = await fs.stat(absolutePath);
  return {absolutePath, name, stats};
}

async function directories(basePath: string): Promise<FsEntry[]> {
  const names = await fs.readdir(basePath);
  const entries = await Promise.all(names.map(name => fsEntry(basePath, name)))
  return entries.filter(({stats}) => stats.isDirectory())
}

export type Submission = DmgSubmission | SgbSubmission |
  MgbSubmission | MglSubmission | Sgb2Submission |
  CgbSubmission | AgbSubmission | AgsSubmission | GbsSubmission | OxySubmission;

export interface Photo {
  path: string;
  name: string;
  stats: fs.Stats;
}

export interface DmgSubmission {
  type: 'dmg';
  title: string;
  slug: string;
  contributor: string;
  metadata: DmgMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    mainboardFront?: Photo;
    mainboardBack?: Photo;
    lcdBoardFront?: Photo;
    lcdBoardBack?: Photo;
    powerBoardFront?: Photo,
    powerBoardBack?: Photo;
    jackBoardFront?: Photo;
    jackBoardBack?: Photo;
  };
}

export interface SgbSubmission {
  type: 'sgb';
  title: string;
  slug: string;
  contributor: string;
  metadata: SgbMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface MgbSubmission {
  type: 'mgb';
  title: string;
  slug: string;
  contributor: string;
  metadata: MgbMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface MglSubmission {
  type: 'mgl';
  title: string;
  slug: string;
  contributor: string;
  metadata: MglMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface Sgb2Submission {
  type: 'sgb2';
  title: string;
  slug: string;
  contributor: string;
  metadata: Sgb2Metadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface CgbSubmission {
  type: 'cgb';
  title: string;
  slug: string;
  contributor: string;
  metadata: CgbMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface AgbSubmission {
  type: 'agb';
  title: string;
  slug: string;
  contributor: string;
  metadata: AgbMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface AgsSubmission {
  type: 'ags';
  title: string;
  slug: string;
  contributor: string;
  metadata: AgsMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface GbsSubmission {
  type: 'gbs';
  title: string;
  slug: string;
  contributor: string;
  metadata: GbsMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export interface OxySubmission {
  type: 'oxy';
  title: string;
  slug: string;
  contributor: string;
  metadata: OxyMetadata;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  };
}

export async function crawlDataDirectory(path: string): Promise<Submission[]> {
  interface Submission {
    contributor: FsEntry;
    model: FsEntry;
    unit: FsEntry;
  }
  const contributors = await directories(path);
  const submissions = R.flatten<Submission>(await Promise.all(contributors.map(async contributor => {
    const models = await directories(contributor.absolutePath);
    return R.flatten<Submission>(await Promise.all(models.map(async model => {
      const units = await directories(model.absolutePath);
      return units.map(unit => ({contributor, model, unit}))
    })));
  })));
  return rejectNil(await Promise.all(submissions.map(async ({contributor, model, unit}) => {
    switch(model.name) {
      case 'DMG': {
        const metadata = await readMetadata<DmgMetadata>(unit, DmgMetadata.schema);
        if (metadata) {
          return crawlDMG(contributor.name, unit, metadata);
        }
        break;
      }
      case 'SGB': {
        const metadata = await readMetadata<SgbMetadata>(unit, SgbMetadata.schema);
        if (metadata) {
          return crawlSGB(contributor.name, unit, metadata);
        }
        break;
      }
      case 'MGB': {
        const metadata = await readMetadata<MgbMetadata>(unit, MgbMetadata.schema);
        if (metadata) {
          return crawlMGB(contributor.name, unit, metadata);
        }
        break;
      }
      case 'MGL': {
        const metadata = await readMetadata<MglMetadata>(unit, MglMetadata.schema);
        if (metadata) {
          return crawlMGL(contributor.name, unit, metadata);
        }
        break;
      }
      case 'SGB2': {
        const metadata = await readMetadata<Sgb2Metadata>(unit, Sgb2Metadata.schema);
        if (metadata) {
          return crawlSGB2(contributor.name, unit, metadata);
        }
        break;
      }
      case 'CGB': {
        const metadata = await readMetadata<CgbMetadata>(unit, CgbMetadata.schema);
        if (metadata) {
          return crawlCGB(contributor.name, unit, metadata);
        }
        break;
      }
      case 'AGB': {
        const metadata = await readMetadata<AgbMetadata>(unit, AgbMetadata.schema);
        if (metadata) {
          return crawlAGB(contributor.name, unit, metadata);
        }
        break;
      }
      case 'AGS': {
        const metadata = await readMetadata<AgsMetadata>(unit, AgsMetadata.schema);
        if (metadata) {
          return crawlAGS(contributor.name, unit, metadata);
        }
        break;
      }
      case 'GBS': {
        const metadata = await readMetadata<GbsMetadata>(unit, GbsMetadata.schema);
        if (metadata) {
          return crawlGBS(contributor.name, unit, metadata);
        }
        break;
      }
      case 'OXY': {
        const metadata = await readMetadata<OxyMetadata>(unit, OxyMetadata.schema);
        if (metadata) {
          return crawlOXY(contributor.name, unit, metadata);
        }
        break;
      }
      default: {
        console.warn(`Skipping unknown model directory ${model.absolutePath}`);
      }
    }
    return undefined
  })));
}

async function readMetadata<T extends Metadata>(unit: FsEntry, schema: Joi.Schema): Promise<T | undefined> {
  const metadataPath = path.resolve(unit.absolutePath, 'metadata.json');
  const metadata = JSON.parse(await fs.readFile(metadataPath, 'utf-8'));
  const validationResult = Joi.validate(metadata, schema);
  if (validationResult.error) {
    console.error(validationResult.error.annotate());
    return undefined;
  }
  return validationResult.value;
}

async function crawlDMG(contributor: string, unit: FsEntry, metadata: DmgMetadata): Promise<DmgSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    mainboardFront: await fetchPhoto(unit, '03_mainboard_front.jpg'),
    mainboardBack: await fetchPhoto(unit, '04_mainboard_back.jpg'),
    lcdBoardFront: await fetchPhoto(unit, '05_lcd_board_front.jpg'),
    lcdBoardBack: await fetchPhoto(unit, '06_lcd_board_back.jpg'),
    powerBoardFront: await fetchPhoto(unit, '07_power_board_front.jpg'),
    powerBoardBack: await fetchPhoto(unit, '08_power_board_back.jpg'),
    jackBoardFront: await fetchPhoto(unit, '09_jack_board_front.jpg'),
    jackBoardBack: await fetchPhoto(unit, '10_jack_board_back.jpg'),
  };

  return {type: 'dmg', title, slug, contributor, metadata, photos};
}

async function crawlSGB(contributor: string, unit: FsEntry, metadata: SgbMetadata): Promise<SgbSubmission> {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb', title, slug, contributor, metadata, photos};
}

async function crawlMGB(contributor: string, unit: FsEntry, metadata: MgbMetadata): Promise<MgbSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'mgb', title, slug, contributor, metadata, photos};
}

async function crawlMGL(contributor: string, unit: FsEntry, metadata: MglMetadata): Promise<MglSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'mgl', title, slug, contributor, metadata, photos};
}

async function crawlSGB2(contributor: string, unit: FsEntry, metadata: Sgb2Metadata): Promise<Sgb2Submission> {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb2', title, slug, contributor, metadata, photos};
}

async function crawlCGB(contributor: string, unit: FsEntry, metadata: CgbMetadata): Promise<CgbSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'cgb', title, slug, contributor, metadata, photos};
}

async function crawlAGB(contributor: string, unit: FsEntry, metadata: AgbMetadata): Promise<AgbSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'agb', title, slug, contributor, metadata, photos};
}

async function crawlAGS(contributor: string, unit: FsEntry, metadata: AgsMetadata): Promise<AgsSubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'ags', title, slug, contributor, metadata, photos};
}

async function crawlGBS(contributor: string, unit: FsEntry, metadata: GbsMetadata): Promise<GbsSubmission> {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'gbs', title, slug, contributor, metadata, photos};
}

async function crawlOXY(contributor: string, unit: FsEntry, metadata: OxyMetadata): Promise<OxySubmission> {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: await fetchPhoto(unit, '01_front.jpg'),
    back: await fetchPhoto(unit, '02_back.jpg'),
    pcbFront: await fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: await fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'oxy', title, slug, contributor, metadata, photos};
}

async function fetchPhoto(entry: FsEntry, name: string): Promise<Photo | undefined> {
  const absolutePath = path.resolve(entry.absolutePath, name);
  try {
    const stats = await fs.stat(absolutePath)
    return {
      path: absolutePath,
      name,
      stats
    };
  } catch (e) {
    if (e.code === 'ENOENT') {
      return undefined
    }
    throw e
  }
}
