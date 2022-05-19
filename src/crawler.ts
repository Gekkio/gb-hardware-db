import * as fs from 'fs-extra'

import { CartridgeMetadata } from './metadata'

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
  sort_group: string | undefined
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
