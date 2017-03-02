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
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/sgb/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front
              ? <img
                src={`/static/sgb/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/sgb/${slug}_thumbnail_50.jpg 50w, /static/sgb/${slug}_thumbnail_80.jpg 80w`}
                sizes="(min-width: 1000px) 80px, 50px"
                role="presentation" />
              : null
            }
          </div>
          <div className="submission-list-item__id">
            <div className="submission-list-item__title">{title}</div>
            <aside className="submission-list-item__contributor">{contributor}</aside>
          </div>
        </a>
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
