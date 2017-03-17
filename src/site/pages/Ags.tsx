import * as React from 'react';

import {AgsSubmission} from '../../crawler';
import * as format from '../format';
import ConsoleListingChip from '../components/ConsoleListingChip';

interface Props {
  submissions: AgsSubmission[];
}

export default function Ags({submissions}: Props) {
  return (
    <article>
      <h2>Game Boy Advance SP (AGS)</h2>
      <table>
        <thead>
        <tr>
          <th>ID</th>
          <th>Board</th>
          <th>CPU (U1)</th>
        </tr>
        </thead>
        <tbody>
        {submissions.map(submission =>
          <Submission key={submission.slug} submission={submission} />
        )}
        </tbody>
      </table>
    </article>
  )
}

function Submission({submission: {contributor, slug, title, metadata}}: {submission: AgsSubmission}) {
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link">
          <div className="submission-list-item__id">
            <div className="submission-list-item__title">{title}</div>
            <aside className="submission-list-item__contributor">{contributor}</aside>
          </div>
        </a>
      </td>
      <td>
        <div>{metadata.mainboard.type}</div>
        <div>{format.short.calendar(metadata.mainboard)}</div>
      </td>
      <ConsoleListingChip chip={metadata.mainboard.cpu} />
    </tr>
  )
}
