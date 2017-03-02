import * as React from 'react';

import {Photo, OxySubmission} from '../../crawler';
import {Chip, OxyMetadata} from '../../metadata';
import {formatYearMonth, formatYearWeek} from '../format';

export default function OxyConsole({submission}: {submission: OxySubmission}) {
  return (
    <article className="page-oxy-console">
      <h2>{`OXY: ${submission.title}`}</h2>
      <div className="page-oxy-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Color</dt>
        <dd>{submission.metadata.color || '????'}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="page-oxy-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Manufacture date</dt>
        <dd>{formatYearMonth(submission.metadata.mainboard)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{submission.metadata.mainboard.circled_letters || '??'}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: OxySubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/oxy/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: OxyMetadata) {
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
      {renderChip('U2', '', mainboard.u2)}
      {renderChip('U4', '', mainboard.u4)}
      {renderChip('U5', '', mainboard.u5)}
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
      <td>{chip.type || '????'}</td>
      <td>{formatYearWeek(chip)}</td>
      <td>{chip.label}</td>
    </tr>
  )
}
