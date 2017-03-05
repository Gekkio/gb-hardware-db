import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as jimp from 'jimp';
import * as path from 'path';
import * as winston from 'winston';

import * as files from '../util/files';
import {Photo, Submission} from '../crawler';

export default function processPhotos<T extends Submission>(submission: T): Bluebird<any> {
  const photos = R.values(submission.photos).filter(x => !!x) as Photo[];
  if (photos.length === 0) {
    winston.warn(`[${submission.type}] ${submission.slug}: no photos`);
    return Bluebird.resolve();
  }

  const targetDirectory = path.resolve('build', 'site', 'static', submission.type);
  const thumbnailPhoto = submission.photos.front;

  function writeThumbnail(size: number): Bluebird<void> {
    if (!thumbnailPhoto) {
      winston.warn(`[${submission.type}] ${submission.slug}: thumbnail source is not available`);
      return Bluebird.resolve();
    }
    const target = path.resolve(targetDirectory, `${submission.slug}_thumbnail_${size}.jpg`);
    return files.doIfOutdated(target, thumbnailPhoto.stats, () => {
      return Bluebird.resolve(jimp.read(thumbnailPhoto.path))
        .then(image => {
          image
            .resize(size, jimp.AUTO)
            .write(target);
        })
        .then(() => files.setModificationTime(target, thumbnailPhoto.stats.mtime))
        .tap(() => winston.debug(`[${submission.type}] ${submission.slug}: wrote thumbnail ${target}`));
    })
  }

  return files.ensureDir(targetDirectory)
    .then(() => Bluebird.all(
      photos.map(photo => {
        const target = path.resolve(targetDirectory, `${submission.slug}_${photo.name}`);
        return files.doIfOutdated(target, photo.stats, () => {
          return files.copy(photo.path, target, {preserveTimestamps: true})
            .tap(() => winston.debug(`[${submission.type}] ${submission.slug}: copied photo ${target}`));
        })
      }).concat([
        writeThumbnail(80),
        writeThumbnail(50),
      ])
    ))
}
