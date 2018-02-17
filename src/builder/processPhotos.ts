import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as fs from 'fs-extra';
import * as jimp from 'jimp';
import * as path from 'path';
import * as winston from 'winston';

import * as files from '../util/files';
import {Photo} from '../crawler';
import {rejectNil} from '../util/arrays';

interface Input<P extends {front?: Photo}> {
  type: string,
  slug: string,
  photos: P,
}

export default async function processPhotos<P extends {front?: Photo}, T extends Input<P>>(input: T): Promise<void> {
  const photos = rejectNil(R.values(input.photos));
  if (photos.length === 0) {
    winston.warn(`[${input.type}] ${input.slug}: no photos`);
    return
  }

  const targetDirectory = path.resolve('build', 'site', 'static', input.type);
  const thumbnailPhoto = input.photos.front;

  const writeThumbnail = async (size: number) => {
    if (!thumbnailPhoto) {
      winston.warn(`[${input.type}] ${input.slug}: thumbnail source is not available`);
      return
    }
    const target = path.resolve(targetDirectory, `${input.slug}_thumbnail_${size}.jpg`);
    if (await files.isOutdated(target, thumbnailPhoto.stats)) {
      const image = await jimp.read(thumbnailPhoto.path);
      await Bluebird.fromNode(cb => image.resize(size, jimp.AUTO).quality(80).write(target, cb));
      await files.setModificationTime(target, thumbnailPhoto.stats);
      winston.debug(`[${input.type}] ${input.slug}: wrote thumbnail ${target}`);
    }
  };

  await fs.ensureDir(targetDirectory);
  for (const photo of photos) {
    const target = path.resolve(targetDirectory, `${input.slug}_${photo.name}`);
    if (await files.isOutdated(target, photo.stats)) {
      await fs.copy(photo.path, target, {preserveTimestamps: true});
      winston.debug(`[${input.type}] ${input.slug}: copied photo ${target}`);
    }
  }
  await writeThumbnail(80);
  await writeThumbnail(50);
}
