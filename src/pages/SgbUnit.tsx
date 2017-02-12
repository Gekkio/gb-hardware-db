import * as React from 'react';

import {Photo, SgbSubmission} from '../crawler';

export default function SgbUnit({submission}: {submission: SgbSubmission}) {
  return (
    <article>
      <h2>{`SGB: ${submission.title}`}</h2>
      {renderPhoto(submission, submission.photos.front)}
    </article>
  )
}

function renderPhoto(submission: SgbSubmission, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  return (
      <img src={`/static/sgb/${submission.slug}_${photo.name}`} />
  )
}
