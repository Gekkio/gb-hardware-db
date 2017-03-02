import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as fs from 'fs-extra';
import * as jimp from 'jimp';
import * as path from 'path';
import * as winston from 'winston';

import {Photo, Submission} from '../crawler';

const copy: (src: string, dst: string, opts: fs.CopyOptions) => Bluebird<void> = Bluebird.promisify(fs.copy) as any;
const ensureDir: (path: string) => Bluebird<void> = Bluebird.promisify(fs.ensureDir) as any;

export default function processPhotos<T extends Submission>(submission: T): Bluebird<any> {
  const photos = R.values(submission.photos).filter(x => !!x) as Photo[];
  if (photos.length === 0) {
    winston.debug(`[${submission.slug}]: no photos`);
    return Bluebird.resolve();
  }

  const targetDirectory = path.resolve('build', 'site', 'static', submission.type);
  const thumbnailPhoto = submission.photos.front;

  function writeThumbnail(size: number): Bluebird<void> {
    if (!thumbnailPhoto) {
      winston.warn(`[${submission.slug}]: thumbnail source is not available`);
      return Bluebird.resolve();
    }
    const target = path.resolve(targetDirectory, `${submission.slug}_thumbnail_${size}.jpg`);
    return Bluebird.resolve(jimp.read(thumbnailPhoto.path))
      .then(image => {
        image
          .contain(size, size)
          .background(0xFFFFFFFF)
          .write(target);
      })
      .tap(() => winston.debug(`[${submission.slug}]: wrote thumbnail ${target}`));
  }

  return ensureDir(targetDirectory)
    .then(() => Bluebird.all(
      photos.map(photo => {
        const target = path.resolve(targetDirectory, `${submission.slug}_${photo.name}`);
        return copy(photo.path, target, {preserveTimestamps: true})
          .tap(() => winston.debug(`[$submission.slug}]: copied photo ${target}`));
      }).concat([
        writeThumbnail(80),
        writeThumbnail(50),
      ])
    ))
}
