import * as React from 'react'

import { CartridgeSubmission, Photo } from '../../crawler'
import * as format from '../format'
import ConsoleListingChip from '../components/ConsoleListingChip'
import { CartLayout, GameConfig, gameLayouts } from '../../config'

interface Props {
  type: string
  cfg: GameConfig
  submissions: CartridgeSubmission[]
}

export default function Game({ type, cfg, submissions }: Props) {
  const layout = gameLayouts[cfg.layouts[0]]
  return (
    <article>
      <h2>{cfg.name}</h2>
      <table>
        <thead>
          <tr>
            <th>Entry</th>
            <th>Release</th>
            <th>Board</th>
            {layout.chips.map(({ designator, name }) => (
              <th key={designator}>{`${name} (${designator})`}</th>
            ))}
            <th>Photos</th>
          </tr>
        </thead>
        <tbody>
          {submissions.map(submission => (
            <Submission key={submission.slug} type={type} layout={layout} submission={submission} />
          ))}
        </tbody>
      </table>
    </article>
  )
}

function Submission({
  type,
  layout,
  submission: { contributor, slug, title, metadata, photos },
}: {
  type: string
  layout: CartLayout
  submission: CartridgeSubmission
}) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/cartridges/${type}/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front ? (
              <img
                src={`/static/${type}/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/${type}/${slug}_thumbnail_50.jpg 50w, /static/${type}/${slug}_thumbnail_80.jpg 80w`}
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
      <td>{metadata.code}</td>
      <td>
        <div>{metadata.board.type}</div>
        <div>{format.short.calendar(metadata.board)}</div>
      </td>
      {layout.chips.map(({ designator, key }) => (
        <ConsoleListingChip key={designator} chip={metadata.board[key]} />
      ))}
      <td>
        {renderPhoto(type, slug, 'Front', photos.front)}
        {renderPhoto(type, slug, 'Back', photos.back)}
        {renderPhoto(type, slug, 'PCB front', photos.pcbFront)}
        {renderPhoto(type, slug, 'PCB back', photos.pcbBack)}
      </td>
    </tr>
  )
}

function renderPhoto(type: string, slug: string, label: string, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  return (
    <div>
      <a href={`/static/${type}/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
