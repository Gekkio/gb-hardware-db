import * as fs from 'fs';
import * as path from 'path';
import * as Joi from 'joi';
import * as R from 'ramda';
import * as urlSlug from 'url-slug';

import {Metadata, SgbMetadata, OxyMetadata, Sgb2Metadata} from './metadata';

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

export type Submission = OxySubmission | SgbSubmission | Sgb2Submission;

export interface Photo {
  path: string;
  name: string;
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

export function crawlDataDirectory(path: string): Submission[] {
  const submissions: Submission[] = [];
  directories(path).forEach(contributor => {
    directories(contributor.absolutePath).forEach(model => {
      directories(model.absolutePath).forEach(unit => {
        let submission: Submission | undefined = undefined;
        switch(model.name) {
          case 'SGB': {
            const metadata = readMetadata<SgbMetadata>(unit, SgbMetadata.schema);
            if (metadata) {
              submission = crawlSGB(contributor.name, unit, metadata);
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

function crawlSGB(contributor: string, unit: FsEntry, metadata: SgbMetadata): SgbSubmission {
  const title = `Console #${unit.name} [${contributor}]`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb', title, slug, contributor, metadata, photos};
}

function crawlSGB2(contributor: string, unit: FsEntry, metadata: Sgb2Metadata): Sgb2Submission {
  const title = `Console #${unit.name} [${contributor}]`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb2', title, slug, contributor, metadata, photos};
}

function crawlOXY(contributor: string, unit: FsEntry, metadata: OxyMetadata): OxySubmission {
  const title = `${unit.name} [${contributor}]`;
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
  if (fs.existsSync(absolutePath)) {
    return {
      path: absolutePath,
      name
    };
  }
  return undefined;
}
