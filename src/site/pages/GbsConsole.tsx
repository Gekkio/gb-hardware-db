import * as React from 'react';
import * as R from 'ramda';

import {Photo, GbsSubmission} from '../../crawler';
import {GbsMetadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';
import ConsolePageChipTable from '../components/ConsolePageChipTable';

export default function GbsConsole({submission}: {submission: GbsSubmission}) {
  return (
    <article className="page-console page-console--gbs">
      <h2>{`GBS: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Color</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.color)}</dd>
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
        <dt>Number pair on board</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.number_pair)}</dd>
        <dt>Stamp on board (front, white background)</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.stamp)}</dd>
        <dt>Stamp on board (front)</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.stamp_front)}</dd>
        <dt>Stamp on board (back)</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.stamp_back)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.circled_letters)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: GbsSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/gbs/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: GbsMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="????" chip={mainboard.u4} />
      <ConsolePageChip designator="U5" title="????" chip={mainboard.u5} />
      <ConsolePageChip designator="U6" title="????" chip={mainboard.u6} />
      <ConsolePageChip designator="Y1" title="Crystal" chip={mainboard.crystal} />
    </ConsolePageChipTable>
  )
}
