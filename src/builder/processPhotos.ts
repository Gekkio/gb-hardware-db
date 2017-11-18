import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as fs from 'fs-extra';
import * as jimp from 'jimp';
import * as path from 'path';
import * as winston from 'winston';

import * as files from '../util/files';
import {Photo, Submission} from '../crawler';

export default async function processPhotos<T extends Submission>(submission: T): Promise<void> {
  const photos = R.values(submission.photos).filter(x => !!x) as Photo[];
  if (photos.length === 0) {
    winston.warn(`[${submission.type}] ${submission.slug}: no photos`);
    return
  }

  const targetDirectory = path.resolve('build', 'site', 'static', submission.type);
  const thumbnailPhoto = submission.photos.front;

  const writeThumbnail = async (size: number) => {
    if (!thumbnailPhoto) {
      winston.warn(`[${submission.type}] ${submission.slug}: thumbnail source is not available`);
      return
    }
    const target = path.resolve(targetDirectory, `${submission.slug}_thumbnail_${size}.jpg`);
    if (await files.isOutdated(target, thumbnailPhoto.stats)) {
      const image = await jimp.read(thumbnailPhoto.path);
      await Bluebird.fromNode(cb => image.resize(size, jimp.AUTO).write(target, cb));
      await files.setModificationTime(target, thumbnailPhoto.stats);
      winston.debug(`[${submission.type}] ${submission.slug}: wrote thumbnail ${target}`);
    }
  };

  await fs.ensureDir(targetDirectory);
  for (const photo of photos) {
    const target = path.resolve(targetDirectory, `${submission.slug}_${photo.name}`);
    if (await files.isOutdated(target, photo.stats)) {
      await fs.copy(photo.path, target, {preserveTimestamps: true});
      winston.debug(`[${submission.type}] ${submission.slug}: copied photo ${target}`);
    }
  }
  await writeThumbnail(80);
  await writeThumbnail(50);
}
