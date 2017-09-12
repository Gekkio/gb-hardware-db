import * as React from 'react';

import {MglSubmission, Photo} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

interface Props {
  submissions: MglSubmission[];
}

export default function Mgl({submissions}: Props) {
  return (
    <article>
      <h2>Game Boy Light (MGL)</h2>
      <table>
        <thead>
        <tr>
          <th>ID</th>
          <th>Board</th>
          <th>CPU (U1)</th>
          <th>WRAM (U2)</th>
          <th>Amplifier (U3)</th>
          <th>Regulator (U4)</th>
          <th>Crystal (X1)</th>
          <th>T1</th>
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

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: MglSubmission}) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/mgl/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front
              ? <img
                src={`/static/mgl/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/mgl/${slug}_thumbnail_50.jpg 50w, /static/mgl/${slug}_thumbnail_80.jpg 80w`}
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
        <div>{`Assembled: ${format.short.calendar(metadata)}`}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <ConsoleListingChip chip={metadata.mainboard.amplifier} />
      <ConsoleListingChip chip={metadata.mainboard.regulator} />
      <ConsoleListingChip chip={metadata.mainboard.crystal} />
      <ConsoleListingChip chip={metadata.mainboard.t1} />
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
      <a href={`/static/mgl/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}

