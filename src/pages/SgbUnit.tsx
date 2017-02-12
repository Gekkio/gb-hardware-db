import * as React from 'react';

import {Photo, SgbSubmission} from '../crawler';

export default function SgbUnit({submission}: {submission: SgbSubmission}) {
  return (
    <article className="sgb unit">
      <h2>{`SGB: ${submission.title}`}</h2>
      <div className="unit-photo-big">
        {renderPhoto(submission, submission.photos.front)}
        {renderPhoto(submission, submission.photos.back)}
      </div>
      <div className="unit-photo-big">
        {renderPhoto(submission, submission.photos.pcbFront)}
        {renderPhoto(submission, submission.photos.pcbBack)}
      </div>
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
