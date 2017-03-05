import * as React from 'react';
import * as R from 'ramda';

import {Photo, Sgb2Submission} from '../../crawler';
import {Sgb2Metadata} from '../../metadata';
import * as format from '../format';
import ConsolePageChip from '../components/ConsolePageChip';

export default function Sgb2Console({submission}: {submission: Sgb2Submission}) {
  return (
    <article className="page-sgb2-console">
      <h2>{`SGB2: ${submission.title} [${submission.contributor}]`}</h2>
      <div className="page-sgb2-console__photo">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <dl>
        <dt>Stamp on case</dt>
        <dd>{format.optional(R.identity, submission.metadata.stamp)}</dd>
      </dl>
      <h3>Mainboard</h3>
      <div className="page-sgb2-console__photo">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
      <dl>
        <dt>Board type</dt>
        <dd>{submission.metadata.mainboard.type}</dd>
        <dt>Manufacture date</dt>
        <dd>{format.calendar(submission.metadata.mainboard)}</dd>
        <dt>Circled letter(s) on board</dt>
        <dd>{format.optional(R.identity, submission.metadata.mainboard.circled_letters)}</dd>
        <dt>Letter at top right</dt>
        <dd>{format.optional(R.identity, submission.metadata.mainboard.letter_at_top_right)}</dd>
      </dl>
      <h3>Chips</h3>
      {renderChips(submission.metadata)}
    </article>
  )
}

function renderPhoto(submission: Sgb2Submission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  const url = `/static/sgb2/${submission.slug}_${photo.name}`
  return (
    <a href={url}>
      <img src={url} />
    </a>
  )
}

function renderChips({mainboard}: Sgb2Metadata) {
  return (
    <table>
      <tbody>
        <tr>
          <th />
          <th>Chip</th>
          <th>Type</th>
          <th>Date</th>
          <th>Label</th>
        </tr>
        <ConsolePageChip designator="U1" title="CPU" chip={mainboard.cpu} />
        <ConsolePageChip designator="U2" title="ICD2" chip={mainboard.icd2} />
        <ConsolePageChip designator="U3" title="Work RAM" chip={mainboard.work_ram} />
        <ConsolePageChip designator="U4" title="ROM" chip={mainboard.rom} />
        <ConsolePageChip designator="U5" title="CIC" chip={mainboard.cic} />
        <ConsolePageChip designator="XTAL1" title="Crystal" chip={mainboard.crystal} />
        <ConsolePageChip designator="COIL1" title="Coil" chip={mainboard.coil} />
      </tbody>
    </table>
  )
}
