import * as React from 'react'

import { MgbSubmission, Photo } from '../../crawler'
import * as format from '../format'
import ConsoleListingChip from '../components/ConsoleListingChip'

interface Props {
  submissions: MgbSubmission[]
}

export default function Mgb({ submissions }: Props) {
  return (
    <article>
      <h2>Game Boy Pocket (MGB)</h2>
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
            <th>Photos</th>
          </tr>
        </thead>
        <tbody>
          {submissions.map(submission => (
            <Submission key={submission.slug} submission={submission} />
          ))}
        </tbody>
      </table>
      <h3>Data dumps</h3>
      <a href="/static/export/consoles/mgb.csv">UTF-8 encoded CSV</a>
    </article>
  )
}

function Submission({ submission: { contributor, slug, title, metadata, photos } }: { submission: MgbSubmission }) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/mgb/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front ? (
              <img
                src={`/static/mgb/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/mgb/${slug}_thumbnail_50.jpg 50w, /static/mgb/${slug}_thumbnail_80.jpg 80w`}
                sizes="(min-width: 1000px) 80px, 50px"
                role="presentation"
              />
            ) : (
              <img src={`/mgb_placeholder.svg`} className="submission-list-item__placeholder" role="presentation" />
            )}
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
        {metadata.year && <div>{`Assembled: ${format.short.calendar(metadata)}`}</div>}
        {metadata.release_code && <div>{`Release: ${metadata.release_code}`}</div>}
        {metadata.lcd_panel && metadata.lcd_panel.year && (
          <div>{`LCD panel: ${format.short.calendar(metadata.lcd_panel)}`}</div>
        )}
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <ConsoleListingChip chip={metadata.mainboard.amplifier} />
      <ConsoleListingChip chip={metadata.mainboard.regulator} />
      <ConsoleListingChip chip={metadata.mainboard.crystal} hideType={true} />
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
    return null
  }
  return (
    <div>
      <a href={`/static/mgb/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
