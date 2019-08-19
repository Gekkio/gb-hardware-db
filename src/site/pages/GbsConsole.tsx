import * as React from 'react'

import { GbsSubmission, Photo } from '../../crawler'
import { GbsMetadata } from '../../metadata'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageShell from '../components/ConsolePageShell'
import ConsolePageMainboard from '../components/ConsolePageMainboard'

export default function GbsConsole({ submission }: { submission: GbsSubmission }) {
  return (
    <article className="page-console page-console--gbs">
      <h2>{`GBS: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <ConsolePageShell submission={submission} />
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

function renderPhoto(submission: GbsSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/gbs/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard }: GbsMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="Regulator" chip={mainboard.u5} />
      <ConsolePageChip designator="U6" title="Regulator" chip={mainboard.u6} />
      <ConsolePageChip designator="Y1" title="Crystal" chip={mainboard.crystal} />
    </ConsolePageChipTable>
  )
}
