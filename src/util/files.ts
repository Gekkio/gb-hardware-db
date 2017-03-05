import * as Bluebird from 'bluebird';
import * as fs from 'fs-extra';

export const close: (fd: number) => Bluebird<void> = Bluebird.promisify(fs.close) as any;
export const copy: (src: string, dst: string, opts?: fs.CopyOptions) => Bluebird<void> = Bluebird.promisify(fs.copy) as any;
export const ensureDir: (path: string) => Bluebird<void> = Bluebird.promisify(fs.ensureDir) as any;
export const futimes: (fd: number, atime: number, mtime: number) => Bluebird<void> = Bluebird.promisify(fs.futimes) as any;
export const open: (path: string, flags: string | number, mode?: number) => Bluebird<number> = Bluebird.promisify(fs.open) as any;
export const outputFile: (file: string, data: any) => Bluebird<void> = Bluebird.promisify(fs.outputFile) as any;
export const stat: (path: string) => Bluebird<fs.Stats> = Bluebird.promisify(fs.stat) as any;

export function doIfOutdated(target: string, reference: fs.Stats, f: () => Bluebird<void>): Bluebird<void> {
  return stat(target)
    .then(stats => stats.mtime.getTime() !== reference.mtime.getTime())
    .catch({code: 'ENOENT'}, () => true)
    .then(outdated => {
      if (!outdated) {
        return Promise.resolve();
      }
      return f();
    })
}

export function setModificationTime(path: string, date: Date): Bluebird<void> {
  const atime = NaN
  const mtime = date.getTime() / 1000;
  return Bluebird.using(open(path, 'w').disposer(close), (fd) => futimes(fd, atime, mtime));
}
