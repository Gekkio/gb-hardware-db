import * as React from 'react'

import { AgsSubmission, Photo } from '../../crawler'
import * as format from '../format'
import ConsoleListingChip from '../components/ConsoleListingChip'

interface Props {
  submissions: AgsSubmission[]
}

export default function Ags({ submissions }: Props) {
  return (
    <article>
      <h2>Game Boy Advance SP (AGS)</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Board</th>
            <th>CPU (U1)</th>
            <th>WRAM (U2)</th>
            <th>Amplifier (U3)</th>
            <th>U4</th>
            <th>Battery controller (U5)</th>
            <th>Crystal (X1)</th>
            <th>Photos</th>
          </tr>
        </thead>
        <tbody>
          {submissions.map((submission) => (
            <Submission key={submission.slug} submission={submission} />
          ))}
        </tbody>
      </table>
      <h3>Data dumps</h3>
      <a href="/static/export/consoles/ags.csv">UTF-8 encoded CSV</a>
    </article>
  )
}

function Submission({ submission: { contributor, slug, title, metadata, photos } }: { submission: AgsSubmission }) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/ags/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front ? (
              <img
                src={`/static/ags/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/ags/${slug}_thumbnail_50.jpg 50w, /static/ags/${slug}_thumbnail_80.jpg 80w`}
                sizes="(min-width: 1000px) 80px, 50px"
                role="presentation"
              />
            ) : null}
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
        {metadata.release_code && <div>{`Release: ${metadata.release_code}`}</div>}
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <ConsoleListingChip chip={metadata.mainboard.amplifier} />
      <ConsoleListingChip chip={metadata.mainboard.u4} />
      <ConsoleListingChip chip={metadata.mainboard.u5} />
      <ConsoleListingChip chip={metadata.mainboard.crystal} hideType={true} />
      <td>
        {renderPhoto(slug, 'Front', photos.front)}
        {renderPhoto(slug, 'Top', photos.top)}
        {renderPhoto(slug, 'Back', photos.back)}
        {renderPhoto(slug, 'PCB front', photos.pcbFront)}
        {renderPhoto(slug, 'PCB back', photos.pcbBack)}
      </td>
    </tr>
  )
}

function renderPhoto(slug: string, label: string, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  return (
    <div>
      <a href={`/static/ags/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
