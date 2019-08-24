import * as fs from 'fs-extra'

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
