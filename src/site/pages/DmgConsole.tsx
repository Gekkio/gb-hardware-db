import * as React from 'react'

import { DmgSubmission, Photo } from '../../crawler'
import { DmgMetadata } from '../../metadata'
import * as format from '../format'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageMainboard from '../components/ConsolePageMainboard'
import ConsolePageShell from '../components/ConsolePageShell'

export default function DmgConsole({ submission }: { submission: DmgSubmission }) {
  return (
    <article className="page-console page-console--dmg">
      <h2>{`DMG: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <ConsolePageShell submission={submission} />
      <h3>Mainboard</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.mainboardFront)}
        {renderPhoto(submission, submission.photos.mainboardBack)}
      </div>
      <ConsolePageMainboard submission={submission} />
      <h3>LCD Board</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.lcdBoardFront)}
        {renderPhoto(submission, submission.photos.lcdBoardBack)}
      </div>
      {renderLcdBoardDetails(submission.metadata)}
      <h3>Power Board</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.powerBoardFront)}
        {renderPhoto(submission, submission.photos.powerBoardBack)}
      </div>
      {renderPowerBoardDetails(submission.metadata)}
      <h3>Jack Board</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.jackBoardFront)}
        {renderPhoto(submission, submission.photos.jackBoardBack)}
      </div>
      {renderJackBoardDetails(submission.metadata)}
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderLcdBoardDetails(metadata: DmgMetadata) {
  const { lcd_board } = metadata
  if (!lcd_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{lcd_board.type}</dd>
      {lcd_board.year && (
        <>
          <dt>Manufacture date</dt>
          <dd>{format.calendar(lcd_board)}</dd>
        </>
      )}
      {lcd_board.stamp && (
        <>
          <dt>Stamp</dt>
          <dd>{lcd_board.stamp}</dd>
        </>
      )}
      {lcd_board.circled_letters && (
        <>
          <dt>Circled letter(s) on board</dt>
          <dd>{lcd_board.circled_letters}</dd>
        </>
      )}
      {lcd_board.lcd_panel && lcd_board.lcd_panel.label && (
        <>
          <dt>LCD panel label</dt>
          <dd>{lcd_board.lcd_panel.label}</dd>
        </>
      )}
      {lcd_board.lcd_panel && lcd_board.lcd_panel.year && (
        <>
          <dt>LCD panel date</dt>
          <dd>{format.calendar(lcd_board.lcd_panel)}</dd>
        </>
      )}
    </dl>
  )
}

function renderPowerBoardDetails(metadata: DmgMetadata) {
  const { power_board } = metadata
  if (!power_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{`Type ${power_board.type}`}</dd>
      {power_board.year && (
        <>
          <dt>Manufacture date</dt>
          <dd>{format.calendar(power_board)}</dd>
        </>
      )}
      {power_board.label && (
        <>
          <dt>Label</dt>
          <dd>{power_board.label}</dd>
        </>
      )}
    </dl>
  )
}

function renderJackBoardDetails(metadata: DmgMetadata) {
  const { jack_board } = metadata
  if (!jack_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{jack_board.type}</dd>
      {jack_board.extra_label && (
        <>
          <dt>Extra label</dt>
          <dd>{jack_board.extra_label}</dd>
        </>
      )}
    </dl>
  )
}
function renderPhoto(submission: DmgSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/dmg/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard, lcd_board }: DmgMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="Video RAM" chip={mainboard.video_ram} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="Amplifier" chip={mainboard.amplifier} />
      <ConsolePageChip designator="X1" title="Crystal" chip={mainboard.crystal} />
      <ConsolePageChip
        designator="-"
        title="LCD Column Driver"
        chip={lcd_board && lcd_board.lcd_panel && lcd_board.lcd_panel.column_driver}
      />
      <ConsolePageChip
        designator="-"
        title="LCD Row Driver"
        chip={lcd_board && lcd_board.lcd_panel && lcd_board.lcd_panel.row_driver}
      />
      <ConsolePageChip designator="-" title="LCD Regulator" chip={lcd_board && lcd_board.regulator} />
    </ConsolePageChipTable>
  )
}
