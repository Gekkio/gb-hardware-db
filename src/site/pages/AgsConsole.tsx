import * as React from 'react'

import { AgsSubmission, Photo } from '../../crawler'
import { AgsMetadata } from '../../metadata'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageShell from '../components/ConsolePageShell'
import ConsolePageMainboard from '../components/ConsolePageMainboard'

export default function AgsConsole({ submission }: { submission: AgsSubmission }) {
  return (
    <article className="page-console page-console--ags">
      <h2>{`AGS: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.top)}
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

function renderPhoto(submission: AgsSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/ags/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard }: AgsMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U3" title="Amplifier" chip={mainboard.amplifier} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="Battery controller" chip={mainboard.u5} />
      <ConsolePageChip designator="X1" title="Crystal" chip={mainboard.crystal} />
    </ConsolePageChipTable>
  )
}
