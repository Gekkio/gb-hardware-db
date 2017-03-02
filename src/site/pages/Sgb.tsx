import * as React from 'react';

import {SgbSubmission, Photo} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

interface Props {
  submissions: SgbSubmission[];
}

export default function Sgb({submissions}: Props) {
  return (
    <article>
      <h2>Super Game Boy (SGB)</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Board</th>
            <th>CPU (U1)</th>
            <th>ICD2 (U2)</th>
            <th>WRAM (U3)</th>
            <th>VRAM (U4)</th>
            <th>ROM (U5)</th>
            <th>CIC (U6)</th>
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

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: SgbSubmission}) {
  return (
    <tr>
      <td>
        <div><a href={`/consoles/sgb/${slug}.html`}>{title}</a></div>
        <div><aside>{contributor}</aside></div>
      </td>
      <td>
        <div>{metadata.mainboard.type}</div>
        <div>{format.short.calendar(metadata.mainboard)}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.icd2} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <ConsoleListingChip chip={metadata.mainboard.video_ram} />
      <ConsoleListingChip chip={metadata.mainboard.rom} />
      <ConsoleListingChip chip={metadata.mainboard.cic} />
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
      <a href={`/static/sgb/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
