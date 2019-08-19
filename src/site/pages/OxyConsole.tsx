import * as React from 'react'

import { OxySubmission, Photo } from '../../crawler'
import { OxyMetadata } from '../../metadata'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageShell from '../components/ConsolePageShell'
import ConsolePageMainboard from '../components/ConsolePageMainboard'

export default function OxyConsole({ submission }: { submission: OxySubmission }) {
  return (
    <article className="page-console page-console--oxy">
      <h2>{`OXY: ${submission.title} [${submission.contributor}]`}</h2>
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

function renderPhoto(submission: OxySubmission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/oxy/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard }: OxyMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="????" chip={mainboard.u2} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="????" chip={mainboard.u5} />
    </ConsolePageChipTable>
  )
}
