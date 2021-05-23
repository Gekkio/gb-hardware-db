import * as fs from 'fs-extra'
import util from 'util'

interface NodeError extends Error {
  code: string
}

function isNodeError(e: unknown): e is NodeError {
  return util.types.isNativeError(e) && 'code' in e
}

export async function isOutdated(path: string, reference: fs.Stats): Promise<boolean> {
  try {
    const stats = await fs.stat(path)
    return stats.mtime.getTime() !== reference.mtime.getTime()
  } catch (e: unknown) {
    if (isNodeError(e) && e.code === 'ENOENT') {
      return true
    }
    throw e
  }
}

export async function setModificationTime(path: string, reference: fs.Stats): Promise<void> {
  const atime = new Date().getTime() / 1000
  const mtime = reference.mtimeMs / 1000
  let fd
  try {
    fd = await fs.open(path, 'r+')
    await fs.futimes(fd, atime, mtime)
  } finally {
    if (fd) {
      await fs.close(fd)
    }
  }
}
