import * as R from 'ramda';
import * as React from 'react';

import {Photo, OxySubmission} from '../../crawler';
import {OxyMetadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';
import ConsolePageChipTable from '../components/ConsolePageChipTable';

export default function OxyConsole({submission}: {submission: OxySubmission}) {
  return (
    <article className="page-console page-console--oxy">
      <h2>{`OXY: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Color</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.color)}</dd>
        <dt>Release code</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.release_code)}</dd>
      </dl>
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
        <dt>Circled letter(s) on board</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.circled_letters)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: OxySubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/oxy/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: OxyMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="????" chip={mainboard.u2} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="????" chip={mainboard.u5} />
    </ConsolePageChipTable>
  )
}
