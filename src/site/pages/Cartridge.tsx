import * as React from 'react'
import * as R from 'ramda'

import { CartridgeSubmission, Photo } from '../../crawler'
import { CartridgeMetadata } from '../../metadata'
import * as format from '../format'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import { CartLayout, GameConfig, gameLayouts } from '../../config'

export default function Cartridge({ submission, cfg }: { submission: CartridgeSubmission; cfg: GameConfig }) {
  const layout = gameLayouts[cfg.layout]
  return (
    <article className="page-cartridge">
      <h2>{`${cfg.name}: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-cartridge__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Release</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.code)}</dd>
        <dt>Stamp on case</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.stamp)}</dd>
      </dl>
      <h3>Board</h3>
      <div className="page-cartridge__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Board type</dt>
        <dd>{submission.metadata.board.type}</dd>
        <dt>Manufacture date</dt>
        <dd>{format.calendar(submission.metadata.board)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.board.circled_letters)}</dd>
        <dt>Extra label</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.board.extra_label)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(layout, submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: CartridgeSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/${submission.type}/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}
function renderChips(layout: CartLayout, { board }: CartridgeMetadata) {
  return (
    <ConsolePageChipTable>
      {layout.chips.map(({ designator, name, key }) => (
        <ConsolePageChip key={designator} designator={designator} title={name} chip={board[key]} />
      ))}
    </ConsolePageChipTable>
  )
}
