import * as React from 'react'

import { MgbSubmission, Photo } from '../../crawler'
import { MgbMetadata } from '../../metadata'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageShell from '../components/ConsolePageShell'
import ConsolePageMainboard from '../components/ConsolePageMainboard'
import * as format from '../format'

export default function MgbConsole({ submission }: { submission: MgbSubmission }) {
  const { metadata } = submission
  return (
    <article className="page-console page-console--mgb">
      <h2>{`MGB: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <ConsolePageShell submission={submission} />
      {metadata.lcd_panel && metadata.lcd_panel.label && (
        <>
          <dt>LCD panel label</dt>
          <dd>{metadata.lcd_panel.label}</dd>
        </>
      )}
      {metadata.lcd_panel && metadata.lcd_panel.year && (
        <>
          <dt>LCD panel date</dt>
          <dd>{format.calendar(metadata.lcd_panel)}</dd>
        </>
      )}
      <h3>Mainboard</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <ConsolePageMainboard submission={submission} />
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: MgbSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/mgb/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard, lcd_panel }: MgbMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U3" title="Amplifier" chip={mainboard.amplifier} />
      <ConsolePageChip designator="U4" title="Regulator" chip={mainboard.regulator} />
      <ConsolePageChip designator="X1" title="Crystal" chip={mainboard.crystal} />
      <ConsolePageChip designator="-" title="LCD Column Driver" chip={lcd_panel && lcd_panel.column_driver} />
      <ConsolePageChip designator="-" title="LCD Row Driver" chip={lcd_panel && lcd_panel.row_driver} />
    </ConsolePageChipTable>
  )
}
