import * as React from 'react';

import {Photo, Sgb2Submission} from '../../crawler';
import {Chip, Sgb2Metadata} from '../../metadata';
import {formatYearMonth, formatYearWeek} from '../format';

export default function Sgb2Console({submission}: {submission: Sgb2Submission}) {
  return (
    <article className="page-sgb2-console">
      <h2>{`SGB2: ${submission.title}`}</h2>
      <div className="page-sgb2-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Stamp on case</dt>
        <dd>{submission.metadata.stamp || '??'}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="page-sgb2-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Board type</dt>
        <dd>{submission.metadata.mainboard.type}</dd>
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

function renderPhoto(submission: Sgb2Submission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/sgb2/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: Sgb2Metadata) {
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
      {renderChip('U4', 'ROM', mainboard.rom)}
      {renderChip('U5', 'CIC', mainboard.cic)}
      {renderChip('XTAL1', 'Crystal', mainboard.crystal)}
      {renderChip('COIL1', 'Coil', mainboard.coil)}
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
