import * as R from 'ramda';
import * as React from 'react';

import {DmgSubmission, Photo} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

interface Props {
  submissions: DmgSubmission[];
}

export default function Dmg({submissions}: Props) {
  return (
    <article>
      <h2>Game Boy (DMG)</h2>
      <table>
        <thead>
        <tr>
          <th>ID</th>
          <th>Mainboard</th>
          <th>CPU (U1)</th>
          <th>VRAM (U2)</th>
          <th>WRAM (U3)</th>
          <th>LCD board</th>
          <th>Power board</th>
          <th>Jack board</th>
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

function Submission({submission: {contributor, slug, title, metadata, photos}}: {submission: DmgSubmission}) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/consoles/dmg/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front
              ? <img
                src={`/static/dmg/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/dmg/${slug}_thumbnail_50.jpg 50w, /static/dmg/${slug}_thumbnail_80.jpg 80w`}
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
        <div>{`Assembled: ${format.short.calendar(metadata)}`}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
      <ConsoleListingChip chip={metadata.mainboard.video_ram} />
      <ConsoleListingChip chip={metadata.mainboard.work_ram} />
      <td>
        <div>{format.optional<string>(R.identity, metadata.lcd_board && metadata.lcd_board.type)}</div>
        <div>{format.optional(format.short.calendar, metadata.lcd_board)}</div>
      </td>
      <td>
        <div>{format.optional(v => `Type ${v}`, metadata.power_board && metadata.power_board.type)}</div>
        <div>{format.optional(format.short.calendar, metadata.power_board)}</div>
      </td>
      <td>{format.optional<string>(R.identity, metadata.jack_board && metadata.jack_board.type)}</td>
      <td>
        {renderPhoto(slug, 'Front', photos.front)}
        {renderPhoto(slug, 'Back', photos.back)}
        {renderPhoto(slug, 'Mainboard front', photos.mainboardFront)}
        {renderPhoto(slug, 'Mainboard back', photos.mainboardBack)}
        {renderPhoto(slug, 'LCD board front', photos.lcdBoardFront)}
        {renderPhoto(slug, 'LCD board back', photos.lcdBoardBack)}
        {renderPhoto(slug, 'Power board front', photos.powerBoardFront)}
        {renderPhoto(slug, 'Power board back', photos.powerBoardBack)}
        {renderPhoto(slug, 'Jack board front', photos.jackBoardFront)}
        {renderPhoto(slug, 'Jack board back', photos.jackBoardBack)}
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
      <a href={`/static/dmg/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
