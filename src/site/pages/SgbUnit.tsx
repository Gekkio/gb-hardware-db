import * as React from 'react';
import * as humanDate from 'human-date';

import {Photo, SgbSubmission} from '../../crawler';
import {Chip, SgbMetadata} from '../../metadata';

function formatYearWeek<T extends {year?: number, week?: number}>({year, week}: T): string {
  return `Week ${week || '??'}/${year || '????'}`
}
function formatYearMonth<T extends {year?: number, month?: number}>({year, month}: T): string {
  const monthName = (month && humanDate.monthName(month)) || '??';
  return `${monthName}/${year || '????'}`;
}

export default function SgbUnit({submission}: {submission: SgbSubmission}) {
  return (
    <article className="sgb unit">
      <h2>{`SGB: ${submission.title}`}</h2>
      <div className="unit-photo-big">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Stamp on case</dt>
        <dd>{submission.metadata.stamp || '??'}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="unit-photo-big">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Manufacture date</dt>
        <dd>{formatYearMonth(submission.metadata.mainboard)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{submission.metadata.mainboard.circled_letters || '??'}</dd>
        <dt>Letter at top right</dt>
        <dd>{submission.metadata.mainboard.letter_at_top_right || '?'}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: SgbSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/sgb/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: SgbMetadata) {
  return (
    <table>
      <tr>
        <th />
        <th>Chip</th>
        <th>Type</th>
        <th>Date</th>
        <th>Label</th>
      </tr>
      {renderChip('U1', 'CPU', mainboard.cpu)}
      {renderChip('U2', 'ICD2', mainboard.icd2)}
      {renderChip('U3', 'Work RAM', mainboard.work_ram)}
      {renderChip('U4', 'Video RAM', mainboard.video_ram)}
      {renderChip('U5', 'ROM', mainboard.rom)}
      {renderChip('U6', 'CIC', mainboard.cic)}
    </table>
  )
}

function renderChip(designator: string, title: string, chip: Chip | undefined) {
  if (!chip) {
    return null;
  }
  return (
    <tr>
      <td>{designator}</td>
      <td>{title}</td>
      <td>{chip.type}</td>
      <td>{formatYearWeek(chip)}</td>
      <td>{chip.label}</td>
    </tr>
  )
}
