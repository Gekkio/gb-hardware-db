import * as R from 'ramda';
import * as React from 'react';

import {Photo, OxySubmission} from '../../crawler';
import {OxyMetadata} from '../../metadata';
import {formatYearMonth, formatOptional} from '../format';
import ConsolePageChip from '../components/ConsolePageChip';

export default function OxyConsole({submission}: {submission: OxySubmission}) {
  return (
    <article className="page-oxy-console">
      <h2>{`OXY: ${submission.title}`}</h2>
      <div className="page-oxy-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Color</dt>
        <dd>{formatOptional(R.identity, submission.metadata.color)}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="page-oxy-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Manufacture date</dt>
        <dd>{formatYearMonth(submission.metadata.mainboard)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{formatOptional(R.identity, submission.metadata.mainboard.circled_letters)}</dd>
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
    <table>
      <tr>
        <th />
        <th>Chip</th>
        <th>Type</th>
        <th>Date</th>
        <th>Label</th>
      </tr>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="????" chip={mainboard.u2} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="????" chip={mainboard.u5} />
    </table>
  )
}
