import * as React from 'react';

import {Photo, Sgb2Submission} from '../../crawler';
import {formatShortYearMonth, formatShortYearWeek} from '../format';

interface Props {
  submissions: Sgb2Submission[];
}

export default function Sgb2({submissions}: Props) {
  return (
    <article>
      <h2>Super Game Boy 2 (SGB2)</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Board</th>
            <th>CPU (U1)</th>
            <th>ICD2 (U2)</th>
            <th>WRAM (U3)</th>
            <th>ROM (U4)</th>
            <th>CIC (U5)</th>
            <th>Crystal (XTAL1)</th>
            <th>Coil (COIL1)</th>
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

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: Sgb2Submission}) {
  return (
    <tr>
      <td>
        <div><a href={`/consoles/sgb2/${slug}.html`}>{title}</a></div>
        <div><aside>{contributor}</aside></div>
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
        <div>{metadata.mainboard.icd2 && metadata.mainboard.icd2.type || '????'}</div>
        <div>{metadata.mainboard.icd2 && formatShortYearWeek(metadata.mainboard.icd2)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.work_ram && metadata.mainboard.work_ram.type || '????'}</div>
        <div>{metadata.mainboard.work_ram && formatShortYearWeek(metadata.mainboard.work_ram)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.rom && metadata.mainboard.rom.type || '????'}</div>
        <div>{metadata.mainboard.rom && formatShortYearWeek(metadata.mainboard.rom)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.cic && metadata.mainboard.cic.type || '????'}</div>
        <div>{metadata.mainboard.cic && formatShortYearWeek(metadata.mainboard.cic)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.crystal && metadata.mainboard.crystal.type || '????'}</div>
        <div>{metadata.mainboard.crystal && formatShortYearWeek(metadata.mainboard.crystal)}</div>
      </td>
      <td>
        <div>{metadata.mainboard.coil && metadata.mainboard.coil.type || '????'}</div>
        <div>{metadata.mainboard.coil && formatShortYearWeek(metadata.mainboard.coil)}</div>
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
      <a href={`/static/sgb2/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
