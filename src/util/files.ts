import * as fs from 'fs-extra';

export async function isOutdated(path: string, reference: fs.Stats): Promise<boolean> {
  try {
    const stats = await fs.stat(path);
    return stats.mtime.getTime() !== reference.mtime.getTime();
  } catch (e) {
    if (e.code !== 'ENOENT') {
      throw e
    }
    return true
  }
}

export async function setModificationTime(path: string, reference: fs.Stats): Promise<void> {
  const atime = new Date().getTime() / 1000;
  const mtime = reference.mtimeMs / 1000;
  let fd
  try {
    fd = await fs.open(path, 'r+');
    await fs.futimes(fd, atime, mtime);
  } finally {
    if (fd) {
      await fs.close(fd)
    }
  }
}
