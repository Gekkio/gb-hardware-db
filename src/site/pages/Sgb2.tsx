import * as React from 'react';

import {Photo, Sgb2Submission} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

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
        <div>{metadata.mainboard.type}</div>
        <div>{format.short.calendar(metadata.mainboard)}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.icd2} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <ConsoleListingChip chip={metadata.mainboard.rom} />
      <ConsoleListingChip chip={metadata.mainboard.cic} />
      <ConsoleListingChip chip={metadata.mainboard.crystal} />
      <ConsoleListingChip chip={metadata.mainboard.coil} />
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
