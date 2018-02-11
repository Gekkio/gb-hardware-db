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

interface SubmissionBase<T extends string, M extends Metadata, P = DefaultPhotos> {
  type: T;
  title: string;
  slug: string;
  contributor: string;
  metadata: M;
  photos: P;
}

export interface DefaultPhotos {
  front?: Photo;
  back?: Photo;
  pcbFront?: Photo;
  pcbBack?: Photo;
}

export interface DmgPhotos {
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
}

export type DmgSubmission = SubmissionBase<'dmg', DmgMetadata, DmgPhotos>;
export type SgbSubmission = SubmissionBase<'sgb', SgbMetadata>;
export type MgbSubmission = SubmissionBase<'mgb', MgbMetadata>;
export type MglSubmission = SubmissionBase<'mgl', MglMetadata>;
export type Sgb2Submission = SubmissionBase<'sgb2', Sgb2Metadata>;
export type CgbSubmission = SubmissionBase<'cgb', CgbMetadata>;
export type AgbSubmission = SubmissionBase<'agb', AgbMetadata>;
export type AgsSubmission = SubmissionBase<'ags', AgsMetadata>;
export type GbsSubmission = SubmissionBase<'gbs', GbsMetadata>;
export type OxySubmission = SubmissionBase<'oxy', OxyMetadata>;

async function crawlDefaultPhotos(unit: FsEntry): Promise<DefaultPhotos> {
  const [front, back, pcbFront, pcbBack] = await Promise.all([
    '01_front.jpg',
    '02_back.jpg',
    '03_pcb_front.jpg',
    '04_pcb_back.jpg',
  ].map(filename => fetchPhoto(unit, filename)));
  return {front, back, pcbFront, pcbBack};
}

async function crawlDmgPhotos(unit: FsEntry): Promise<DmgPhotos> {
  const [
    front, back, mainboardFront, mainboardBack, lcdBoardFront, lcdBoardBack,
    powerBoardFront, powerBoardBack, jackBoardFront, jackBoardBack
  ] = await Promise.all([
    '01_front.jpg',
    '02_back.jpg',
    '03_mainboard_front.jpg',
    '04_mainboard_back.jpg',
    '05_lcd_board_front.jpg',
    '06_lcd_board_back.jpg',
    '07_power_board_front.jpg',
    '08_power_board_back.jpg',
    '09_jack_board_front.jpg',
    '10_jack_board_back.jpg',
  ].map(filename => fetchPhoto(unit, filename)));
  return {
    front, back, mainboardFront, mainboardBack, lcdBoardFront, lcdBoardBack,
    powerBoardFront, powerBoardBack, jackBoardFront, jackBoardBack
  };
}

interface SubmissionPath {
  contributor: FsEntry;
  model: FsEntry;
  unit: FsEntry;
}

function parseUnitName({contributor, unit}: SubmissionPath) {
  if (/^[A-Z]+[0-9]+(-[0-9])?$/.test(unit.name)) {
    return {title: unit.name, slug: unit.name};
  } else if (/^[0-9]+(-[0-9])?$/.test(unit.name)) {
    return {title: `Unit #${unit.name}`, slug: urlSlug(`${contributor.name}-${unit.name}`)};
  } else {
    throw new Error(`Unsupported unit name format "${unit.name}"`);
  }
}

async function crawl<T extends string, M extends Metadata, P = DefaultPhotos>(
  type: T,
  schema: Joi.Schema,
  photoCrawler: (unit: FsEntry) => Promise<P>,
  path: SubmissionPath
): Promise<SubmissionBase<T, M, P> | undefined> {
  const {contributor, unit} = path;
  const metadata = await readMetadata<M>(unit, schema);
  if (!metadata) return undefined;
  const {title, slug} = parseUnitName(path);
  const photos = await photoCrawler(unit);
  return {type, title, slug, contributor: contributor.name, metadata, photos}
}

export async function crawlDataDirectory(path: string): Promise<Submission[]> {
  const contributors = await directories(path);
  const submissions = R.flatten<SubmissionPath>(await Promise.all(contributors.map(async contributor => {
    const models = await directories(contributor.absolutePath);
    return R.flatten<SubmissionPath>(await Promise.all(models.map(async model => {
      const units = await directories(model.absolutePath);
      return units.map(unit => ({contributor, model, unit}))
    })));
  })));
  return rejectNil(await Promise.all(submissions.map(async path => {
    const {model} = path
    switch (model.name) {
      case 'DMG':
        return await crawl<'dmg', DmgMetadata, DmgPhotos>('dmg', DmgMetadata.schema, crawlDmgPhotos, path);
      case 'SGB':
        return await crawl<'sgb', SgbMetadata>('sgb', SgbMetadata.schema, crawlDefaultPhotos, path);
      case 'MGB':
        return await crawl<'mgb', MgbMetadata>('mgb', MgbMetadata.schema, crawlDefaultPhotos, path);
      case 'MGL':
        return await crawl<'mgl', MglMetadata>('mgl', MglMetadata.schema, crawlDefaultPhotos, path);
      case 'SGB2':
        return await crawl<'sgb2', Sgb2Metadata>('sgb2', Sgb2Metadata.schema, crawlDefaultPhotos, path);
      case 'CGB':
        return await crawl<'cgb', CgbMetadata>('cgb', CgbMetadata.schema, crawlDefaultPhotos, path);
      case 'AGB':
        return await crawl<'agb', AgbMetadata>('agb', AgbMetadata.schema, crawlDefaultPhotos, path);
      case 'AGS':
        return await crawl<'ags', AgsMetadata>('ags', AgsMetadata.schema, crawlDefaultPhotos, path);
      case 'GBS':
        return await crawl<'gbs', GbsMetadata>('gbs', GbsMetadata.schema, crawlDefaultPhotos, path);
      case 'OXY':
        return await crawl<'oxy', OxyMetadata>('oxy', OxyMetadata.schema, crawlDefaultPhotos, path);
      default: {
        console.warn(`Skipping unknown model directory ${model.absolutePath}`);
        return undefined
      }
    }
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
