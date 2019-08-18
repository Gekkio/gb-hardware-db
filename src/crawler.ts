import * as R from 'ramda'
import * as fs from 'fs-extra'
import * as path from 'path'
import * as Joi from 'joi'
import * as urlSlug from 'url-slug'

import {
  AgbMetadata,
  AgsMetadata,
  CartridgeMetadata,
  CgbMetadata,
  DmgMetadata,
  GbsMetadata,
  MgbMetadata,
  MglMetadata,
  OxyMetadata,
  Sgb2Metadata,
  SgbMetadata,
} from './metadata'
import { rejectNil } from './util/arrays'

export interface FsEntry {
  absolutePath: string
  name: string
  stats: fs.Stats
}

async function fsEntry(basePath: string, name: string): Promise<FsEntry> {
  const absolutePath = path.resolve(basePath, name)
  const stats = await fs.stat(absolutePath)
  return { absolutePath, name, stats }
}

async function directories(basePath: string): Promise<FsEntry[]> {
  const names = await fs.readdir(basePath)
  const entries = await Promise.all(names.map(name => fsEntry(basePath, name)))
  return entries.filter(({ stats }) => stats.isDirectory())
}

export type ConsoleSubmission =
  | DmgSubmission
  | SgbSubmission
  | MgbSubmission
  | MglSubmission
  | Sgb2Submission
  | CgbSubmission
  | AgbSubmission
  | AgsSubmission
  | GbsSubmission
  | OxySubmission

export type CartridgeSubmission = SubmissionBase<string, CartridgeMetadata>

export interface Photo {
  path: string
  name: string
  stats: fs.Stats
}

interface SubmissionBase<T extends string, M, P = DefaultPhotos> {
  type: T
  title: string
  slug: string
  sortGroup: string | undefined
  contributor: string
  metadata: M
  photos: P
}

export interface DefaultPhotos {
  front?: Photo
  back?: Photo
  pcbFront?: Photo
  pcbBack?: Photo
}

export interface DmgPhotos {
  front?: Photo
  back?: Photo
  mainboardFront?: Photo
  mainboardBack?: Photo
  lcdBoardFront?: Photo
  lcdBoardBack?: Photo
  powerBoardFront?: Photo
  powerBoardBack?: Photo
  jackBoardFront?: Photo
  jackBoardBack?: Photo
}

export interface AgsPhotos extends DefaultPhotos {
  top?: Photo
}

export type DmgSubmission = SubmissionBase<'dmg', DmgMetadata, DmgPhotos>
export type SgbSubmission = SubmissionBase<'sgb', SgbMetadata>
export type MgbSubmission = SubmissionBase<'mgb', MgbMetadata>
export type MglSubmission = SubmissionBase<'mgl', MglMetadata>
export type Sgb2Submission = SubmissionBase<'sgb2', Sgb2Metadata>
export type CgbSubmission = SubmissionBase<'cgb', CgbMetadata>
export type AgbSubmission = SubmissionBase<'agb', AgbMetadata>
export type AgsSubmission = SubmissionBase<'ags', AgsMetadata, AgsPhotos>
export type GbsSubmission = SubmissionBase<'gbs', GbsMetadata>
export type OxySubmission = SubmissionBase<'oxy', OxyMetadata>

async function crawlDefaultPhotos(unit: FsEntry): Promise<DefaultPhotos> {
  const [front, back, pcbFront, pcbBack] = await Promise.all(
    ['01_front.jpg', '02_back.jpg', '03_pcb_front.jpg', '04_pcb_back.jpg'].map(filename => fetchPhoto(unit, filename))
  )
  return { front, back, pcbFront, pcbBack }
}

async function crawlAgsPhotos(unit: FsEntry): Promise<AgsPhotos> {
  const [front, top, back, pcbFront, pcbBack] = await Promise.all(
    ['01_front.jpg', '02_top.jpg', '03_back.jpg', '04_pcb_front.jpg', '05_pcb_back.jpg'].map(filename =>
      fetchPhoto(unit, filename)
    )
  )
  return { front, top, back, pcbFront, pcbBack }
}

interface SubmissionPath {
  contributor: FsEntry
  type: FsEntry
  entry: FsEntry
}

interface SubmissionEntry {
  title: string
  slug: string
  sortGroup: string | undefined
  contributor: string
  entry: FsEntry
}

function consoleSubmissionEntry({ contributor, entry }: SubmissionPath): SubmissionEntry {
  const serialMatch = /^([A-Z]+)[0-9]+(-[0-9])?$/.exec(entry.name)
  if (serialMatch) {
    return {
      title: entry.name,
      slug: entry.name,
      sortGroup: serialMatch[1],
      contributor: contributor.name,
      entry,
    }
  } else if (/^[0-9]+(-[0-9])?$/.test(entry.name)) {
    return {
      title: `Unit #${entry.name}`,
      slug: urlSlug(`${contributor.name}-${entry.name}`),
      sortGroup: undefined,
      contributor: contributor.name,
      entry,
    }
  } else {
    throw new Error(`Unsupported console entry name format "${entry.name}"`)
  }
}

async function crawl<T extends string, M, P = DefaultPhotos>(
  type: T,
  schema: Joi.Schema,
  photoCrawler: (unit: FsEntry) => Promise<P>,
  { title, slug, sortGroup, contributor, entry }: SubmissionEntry
): Promise<SubmissionBase<T, M, P> | undefined> {
  const metadata = await readMetadata<M>(entry, schema)
  if (!metadata) return undefined
  const photos = await photoCrawler(entry)
  return { type, title, slug, sortGroup, contributor, metadata, photos }
}

async function crawlSubmissions(path: string): Promise<SubmissionPath[]> {
  const contributors = await directories(path)
  return R.flatten<SubmissionPath>(
    await Promise.all(
      contributors.map(async contributor => {
        const types = await directories(contributor.absolutePath)
        return R.flatten<SubmissionPath>(
          await Promise.all(
            types.map(async type => {
              const entries = await directories(type.absolutePath)
              return entries.map(entry => ({ contributor, type, entry }))
            })
          )
        )
      })
    )
  )
}

export async function crawlConsoles(path: string): Promise<ConsoleSubmission[]> {
  const submissions = await crawlSubmissions(path)
  return rejectNil(
    await Promise.all(
      submissions.map(async path => {
        const { type } = path
        const entry = consoleSubmissionEntry(path)
        switch (type.name) {
          case 'DMG':
            return undefined
          case 'SGB':
            return undefined
          case 'MGB':
            return await crawl<'mgb', MgbMetadata>('mgb', MgbMetadata.schema, crawlDefaultPhotos, entry)
          case 'MGL':
            return await crawl<'mgl', MglMetadata>('mgl', MglMetadata.schema, crawlDefaultPhotos, entry)
          case 'SGB2':
            return undefined
          case 'CGB':
            return await crawl<'cgb', CgbMetadata>('cgb', CgbMetadata.schema, crawlDefaultPhotos, entry)
          case 'AGB':
            return await crawl<'agb', AgbMetadata>('agb', AgbMetadata.schema, crawlDefaultPhotos, entry)
          case 'AGS':
            return await crawl<'ags', AgsMetadata>('ags', AgsMetadata.schema, crawlAgsPhotos, entry)
          case 'GBS':
            return await crawl<'gbs', GbsMetadata>('gbs', GbsMetadata.schema, crawlDefaultPhotos, entry)
          case 'OXY':
            return await crawl<'oxy', OxyMetadata>('oxy', OxyMetadata.schema, crawlDefaultPhotos, entry)
          default: {
            console.warn(`Skipping unknown console directory ${type.absolutePath}`)
            return undefined
          }
        }
      })
    )
  )
}

async function readMetadata<M>(unit: FsEntry, schema: Joi.Schema): Promise<M | undefined> {
  const metadataPath = path.resolve(unit.absolutePath, 'metadata.json')
  if (!(await fs.pathExists(metadataPath))) {
    console.warn(`Skipping directory without metadata ${unit.absolutePath}`)
    return undefined
  }
  const metadata = JSON.parse(await fs.readFile(metadataPath, 'utf-8'))
  const validationResult = Joi.validate(metadata, schema)
  if (validationResult.error) {
    throw validationResult.error
  }
  return validationResult.value
}

async function fetchPhoto(entry: FsEntry, name: string): Promise<Photo | undefined> {
  const absolutePath = path.resolve(entry.absolutePath, name)
  try {
    const stats = await fs.stat(absolutePath)
    return {
      path: absolutePath,
      name,
      stats,
    }
  } catch (e) {
    if (e.code === 'ENOENT') {
      return undefined
    }
    throw e
  }
}
