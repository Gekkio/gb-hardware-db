import * as React from 'react';
import * as R from 'ramda';

import {Photo, DmgSubmission} from '../../crawler';
import {DmgMetadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';
import ConsolePageChipTable from '../components/ConsolePageChipTable';

export default function DmgConsole({submission}: {submission: DmgSubmission}) {
  return (
    <article className="page-console page-console--dmg">
      <h2>{`DMG: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Color</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.color)}</dd>
        <dt>Screws</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.screws)}</dd>
        <dt>Assembly date</dt>
        <dd>{format.calendar(submission.metadata)}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.mainboardFront)}
        {renderPhoto(submission, submission.photos.mainboardBack)}
      </div>
      <dl>
        <dt>Board type</dt>
        <dd>{submission.metadata.mainboard.type}</dd>
        <dt>Stamp</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.stamp)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.circled_letters)}</dd>
        <dt>Extra label</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.extra_label)}</dd>
      </dl>
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
  if (!metadata.lcd_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{metadata.lcd_board.type}</dd>
      <dt>Manufacture date</dt>
      <dd>{format.calendar(metadata.lcd_board)}</dd>
      <dt>Stamp</dt>
      <dd>{format.optional<string>(R.identity, metadata.lcd_board.stamp)}</dd>
      <dt>Circled letter(s) on board</dt>
      <dd>{format.optional<string>(R.identity, metadata.lcd_board.circled_letters)}</dd>
    </dl>
  )
}

function renderPowerBoardDetails(metadata: DmgMetadata) {
  if (!metadata.power_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{`Type ${metadata.power_board.type}`}</dd>
      <dt>Manufacture date</dt>
      <dd>{format.calendar(metadata.power_board)}</dd>
      <dt>Label</dt>
      <dd>{format.optional<string>(R.identity, metadata.power_board.label)}</dd>
    </dl>
  )
}

function renderJackBoardDetails(metadata: DmgMetadata) {
  if (!metadata.jack_board) {
    return null
  }
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{metadata.jack_board.type}</dd>
      <dt>Extra label</dt>
      <dd>{format.optional<string>(R.identity, metadata.jack_board.extra_label)}</dd>
    </dl>
  )
}
function renderPhoto(submission: DmgSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/dmg/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard, lcd_board}: DmgMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="Video RAM" chip={mainboard.video_ram} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="Amplifier" chip={mainboard.amplifier} />
      <ConsolePageChip designator="X1" title="Crystal" chip={mainboard.crystal} />
      <ConsolePageChip designator="-" title="LCD Column Driver" chip={lcd_board && lcd_board.column_driver} />
      <ConsolePageChip designator="-" title="LCD Row Driver" chip={lcd_board && lcd_board.row_driver} />
      <ConsolePageChip designator="-" title="LCD Regulator" chip={lcd_board && lcd_board.regulator} />
    </ConsolePageChipTable>
  )
}
