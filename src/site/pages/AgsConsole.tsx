import * as React from 'react';

import {AgsSubmission, Photo} from '../../crawler';
import {AgsMetadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';
import ConsolePageChipTable from '../components/ConsolePageChipTable';

export default function AgsConsole({submission}: {submission: AgsSubmission}) {
  return (
    <article className="page-console page-console--ags">
      <h2>{`AGS: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <h3>Mainboard</h3>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Board type</dt>
        <dd>{submission.metadata.mainboard.type}</dd>
        <dt>Manufacture date</dt>
        <dd>{format.calendar(submission.metadata.mainboard)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: AgsSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/ags/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: AgsMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
    </ConsolePageChipTable>
  )
}
