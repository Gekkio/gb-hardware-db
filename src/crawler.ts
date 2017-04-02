import * as fs from 'fs';
import * as path from 'path';
import * as Joi from 'joi';
import * as urlSlug from 'url-slug';

import {
  Metadata, DmgMetadata, SgbMetadata, MgbMetadata, MglMetadata, Sgb2Metadata, CgbMetadata, AgbMetadata, AgsMetadata,
  GbsMetadata, OxyMetadata
} from './metadata';

export interface FsEntry {
  absolutePath: string;
  name: string;
  stats: fs.Stats;
}

function fsEntry(basePath: string, name: string): FsEntry {
  const absolutePath = path.resolve(basePath, name);
  const stats = fs.statSync(absolutePath);
  return {
    absolutePath,
    name,
    stats
  };
}

function directories(basePath: string): FsEntry[] {
  return fs.readdirSync(basePath)
    .map(name => fsEntry(basePath, name))
    .filter(({stats}) => stats.isDirectory());
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

export function crawlDataDirectory(path: string): Submission[] {
  const submissions: Submission[] = [];
  directories(path).forEach(contributor => {
    directories(contributor.absolutePath).forEach(model => {
      directories(model.absolutePath).forEach(unit => {
        let submission: Submission | undefined = undefined;
        switch(model.name) {
          case 'DMG': {
            const metadata = readMetadata<DmgMetadata>(unit, DmgMetadata.schema);
            if (metadata) {
              submission = crawlDMG(contributor.name, unit, metadata);
            }
            break;
          }
          case 'SGB': {
            const metadata = readMetadata<SgbMetadata>(unit, SgbMetadata.schema);
            if (metadata) {
              submission = crawlSGB(contributor.name, unit, metadata);
            }
            break;
          }
          case 'MGB': {
            const metadata = readMetadata<MgbMetadata>(unit, MgbMetadata.schema);
            if (metadata) {
              submission = crawlMGB(contributor.name, unit, metadata);
            }
            break;
          }
          case 'MGL': {
            const metadata = readMetadata<MglMetadata>(unit, MglMetadata.schema);
            if (metadata) {
              submission = crawlMGL(contributor.name, unit, metadata);
            }
            break;
          }
          case 'SGB2': {
            const metadata = readMetadata<Sgb2Metadata>(unit, Sgb2Metadata.schema);
            if (metadata) {
              submission = crawlSGB2(contributor.name, unit, metadata);
            }
            break;
          }
          case 'CGB': {
            const metadata = readMetadata<CgbMetadata>(unit, CgbMetadata.schema);
            if (metadata) {
              submission = crawlCGB(contributor.name, unit, metadata);
            }
            break;
          }
          case 'AGB': {
            const metadata = readMetadata<AgbMetadata>(unit, AgbMetadata.schema);
            if (metadata) {
              submission = crawlAGB(contributor.name, unit, metadata);
            }
            break;
          }
          case 'AGS': {
            const metadata = readMetadata<AgsMetadata>(unit, AgsMetadata.schema);
            if (metadata) {
              submission = crawlAGS(contributor.name, unit, metadata);
            }
            break;
          }
          case 'GBS': {
            const metadata = readMetadata<GbsMetadata>(unit, GbsMetadata.schema);
            if (metadata) {
              submission = crawlGBS(contributor.name, unit, metadata);
            }
            break;
          }
          case 'OXY': {
            const metadata = readMetadata<OxyMetadata>(unit, OxyMetadata.schema);
            if (metadata) {
              submission = crawlOXY(contributor.name, unit, metadata);
            }
            break;
          }
          default: {
            console.warn(`Skipping unknown model directory ${model.absolutePath}`);
          }
        }
        if (submission) {
          submissions.push(submission);
        }
      });
    })
  });
  return submissions;
}

function readMetadata<T extends Metadata>(unit: FsEntry, schema: Joi.Schema): T | undefined {
  const metadataPath = path.resolve(unit.absolutePath, 'metadata.json');
  const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf-8'));
  const validationResult = Joi.validate(metadata, schema);
  if (validationResult.error) {
    console.error(validationResult.error.annotate());
    return undefined;
  }
  return validationResult.value;
}

function crawlDMG(contributor: string, unit: FsEntry, metadata: DmgMetadata): DmgSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
  };

  return {type: 'dmg', title, slug, contributor, metadata, photos};
}

function crawlSGB(contributor: string, unit: FsEntry, metadata: SgbMetadata): SgbSubmission {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb', title, slug, contributor, metadata, photos};
}

function crawlMGB(contributor: string, unit: FsEntry, metadata: MgbMetadata): MgbSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'mgb', title, slug, contributor, metadata, photos};
}

function crawlMGL(contributor: string, unit: FsEntry, metadata: MglMetadata): MglSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'mgl', title, slug, contributor, metadata, photos};
}

function crawlSGB2(contributor: string, unit: FsEntry, metadata: Sgb2Metadata): Sgb2Submission {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb2', title, slug, contributor, metadata, photos};
}

function crawlCGB(contributor: string, unit: FsEntry, metadata: CgbMetadata): CgbSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'cgb', title, slug, contributor, metadata, photos};
}

function crawlAGB(contributor: string, unit: FsEntry, metadata: AgbMetadata): AgbSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'agb', title, slug, contributor, metadata, photos};
}

function crawlAGS(contributor: string, unit: FsEntry, metadata: AgsMetadata): AgsSubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'ags', title, slug, contributor, metadata, photos};
}

function crawlGBS(contributor: string, unit: FsEntry, metadata: GbsMetadata): GbsSubmission {
  const title = `Unit #${unit.name}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'gbs', title, slug, contributor, metadata, photos};
}

function crawlOXY(contributor: string, unit: FsEntry, metadata: OxyMetadata): OxySubmission {
  const title = unit.name;
  const slug = unit.name;
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'oxy', title, slug, contributor, metadata, photos};
}

function fetchPhoto(entry: FsEntry, name: string): Photo | undefined {
  const absolutePath = path.resolve(entry.absolutePath, name);
  try {
    const stats = fs.statSync(absolutePath)
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
