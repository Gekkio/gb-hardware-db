import * as React from 'react';
import {OxySubmission, Photo} from '../../crawler';
import {formatShortYearMonth, formatShortYearWeek} from '../format';

interface Props {
  submissions: OxySubmission[];
}

export default function Oxy({submissions}: Props) {
  return (
    <article>
      <h2>Game Boy Micro (OXY)</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Board</th>
            <th>CPU (U1)</th>
            <th>U2</th>
            <th>U4</th>
            <th>U5</th>
            <th>Photos</th>
          </tr>
        </thead>
        <tbody>
          {submissions.map(submission =>
            <Submission key={submission.slug} submission={submission} />
          )}
        </tbody>
      </table>
    </article>
  )
}

function Submission({submission: {slug, title, metadata, photos}}: {submission: OxySubmission}) {
  return (
    <tr>
      <td>
        <a href={`/consoles/oxy/${slug}.html`}>{title}</a>
      </td>
      <td>
        <div>{metadata.mainboard && metadata.mainboard.type}</div>
        <div>{formatShortYearMonth(metadata.mainboard)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.cpu && metadata.mainboard.cpu.type || '????'}</div>
        <div>{metadata.mainboard.cpu && formatShortYearWeek(metadata.mainboard.cpu)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.u2 && metadata.mainboard.u2.type || '????'}</div>
        <div>{metadata.mainboard.u2 && formatShortYearWeek(metadata.mainboard.u2)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.u4 && metadata.mainboard.u4.type || '????'}</div>
        <div>{metadata.mainboard.u4 && formatShortYearWeek(metadata.mainboard.u4)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.u5 && metadata.mainboard.u5.type || '????'}</div>
        <div>{metadata.mainboard.u5 && formatShortYearWeek(metadata.mainboard.u5)}</div>
      </td>
      <td>
        {renderPhoto(slug, 'Front', photos.front)}
        {renderPhoto(slug, 'Back', photos.back)}
        {renderPhoto(slug, 'PCB front', photos.pcbFront)}
        {renderPhoto(slug, 'PCB back', photos.pcbBack)}
      </td>
    </tr>
  )
}

function renderPhoto(slug: string, label: string, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  return (
    <div>
      <a href={`/static/oxy/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
