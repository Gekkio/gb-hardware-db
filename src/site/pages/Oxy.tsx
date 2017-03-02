import * as React from 'react';

import {OxySubmission, Photo} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

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

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: OxySubmission}) {
  return (
    <tr>
      <td>
        <div><a href={`/consoles/oxy/${slug}.html`}>{title}</a></div>
        <div><aside>{contributor}</aside></div>
      </td>
      <td>
        <div>{metadata.mainboard.type}</div>
        <div>{format.short.calendar(metadata.mainboard)}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.u2} />
      <ConsoleListingChip chip={metadata.mainboard.u4} />
      <ConsoleListingChip chip={metadata.mainboard.u5} />
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
