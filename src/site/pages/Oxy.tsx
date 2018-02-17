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
      <h3>Data dumps</h3>
      <a href="/static/export/consoles/oxy.csv">UTF-8 encoded CSV</a>
    </article>
  )
}

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: OxySubmission}) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/oxy/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front
              ? <img
                src={`/static/oxy/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/oxy/${slug}_thumbnail_50.jpg 50w, /static/oxy/${slug}_thumbnail_80.jpg 80w`}
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
