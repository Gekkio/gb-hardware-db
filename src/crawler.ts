import * as fs from 'fs';
import * as path from 'path';
import * as R from 'ramda';
import * as urlSlug from 'url-slug';

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

export type Submission = SgbSubmission;

export interface Photo {
  path: string;
  name: string;
}

export interface SgbSubmission {
  type: 'sgb';
  title: string;
  slug: string;
  contributor: string;
  photos: {
    front?: Photo;
    back?: Photo;
    pcbFront?: Photo;
    pcbBack?: Photo;
  }
}

const modelsByCode = ['SGB'];

export function crawlDataDirectory(path: string): Submission[] {
  const submissions: Submission[] = [];
  directories(path).forEach(contributor => {
    directories(contributor.absolutePath).forEach(model => {
      directories(model.absolutePath).forEach(unit => {
        let submission: Submission | undefined = undefined;
        switch(model.name) {
          case 'SGB': {
            submission = crawlSGB(contributor.name, unit);
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

function crawlSGB(contributor: string, unit: FsEntry): SgbSubmission {
  const title = `Unit #${unit.name} by ${contributor}`;
  const slug = urlSlug(`${contributor}-${unit.name}`);
  const photos = {
    front: fetchPhoto(unit, '01_front.jpg'),
    back: fetchPhoto(unit, '02_back.jpg'),
    pcbFront: fetchPhoto(unit, '03_pcb_front.jpg'),
    pcbBack: fetchPhoto(unit, '04_pcb_back.jpg'),
  };

  return {type: 'sgb', title, slug, contributor, photos};
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
