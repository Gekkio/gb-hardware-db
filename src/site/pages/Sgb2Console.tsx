import * as React from 'react'

import { Photo, Sgb2Submission } from '../../crawler'
import { Sgb2Metadata } from '../../metadata'
import ConsolePageChip from '../components/ConsolePageChip'
import ConsolePageChipTable from '../components/ConsolePageChipTable'
import ConsolePageShell from '../components/ConsolePageShell'
import ConsolePageMainboard from '../components/ConsolePageMainboard'

export default function Sgb2Console({ submission }: { submission: Sgb2Submission }) {
  return (
    <article className="page-console page-console--sgb2">
      <h2>{`SGB2: ${submission.title} [${submission.contributor}]`}</h2>
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

function renderPhoto(submission: Sgb2Submission, photo: Photo | undefined) {
  if (!photo) {
    return null
  }
  const url = `/static/sgb2/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({ mainboard }: Sgb2Metadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="ICD2" chip={mainboard.icd2} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="ROM" chip={mainboard.rom} />
      <ConsolePageChip designator="U5" title="CIC" chip={mainboard.cic} />
      <ConsolePageChip designator="XTAL1" title="Crystal" chip={mainboard.crystal} />
      <ConsolePageChip designator="COIL1" title="Coil" chip={mainboard.coil} />
    </ConsolePageChipTable>
  )
}
