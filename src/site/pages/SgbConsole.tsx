import * as React from 'react';
import * as R from 'ramda';

import {Photo, SgbSubmission} from '../../crawler';
import {SgbMetadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';
import ConsolePageChipTable from '../components/ConsolePageChipTable';

export default function SgbConsole({submission}: {submission: SgbSubmission}) {
  return (
    <article className="page-console page-console--sgb">
      <h2>{`SGB: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Stamp on case</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.stamp)}</dd>
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
        <dt>Letter at top right</dt>
        <dd>{format.optional<string>(R.identity, submission.metadata.mainboard.letter_at_top_right)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: SgbSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/sgb/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: SgbMetadata) {
  return (
    <ConsolePageChipTable>
      <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
      <ConsolePageChip designator="U2" title="ICD2" chip={mainboard.icd2} />
      <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
      <ConsolePageChip designator="U4" title="Work RAM" chip={mainboard.video_ram} />
      <ConsolePageChip designator="U5" title="ROM" chip={mainboard.rom} />
      <ConsolePageChip designator="U6" title="CIC" chip={mainboard.cic} />
    </ConsolePageChipTable>
  )
}
