import * as format from '../format'
import * as React from 'react'
import { ConsoleSubmission } from '../../crawler'

export default function ConsolePageShell({ submission }: { submission: ConsoleSubmission }) {
  const { metadata } = submission
  return (
    <dl>
      {'color' in metadata && metadata.color && (
        <>
          <dt>Color</dt>
          <dd>{metadata.color}</dd>
        </>
      )}
      {'release_code' in metadata && metadata.release_code && (
        <>
          <dt>Release code</dt>
          <dd>{metadata.release_code}</dd>
        </>
      )}
      {'year' in metadata && (
        <>
          <dt>Assembly date</dt>
          <dd>{format.calendar(metadata)}</dd>
        </>
      )}
      {'stamp' in metadata && (
        <>
          <dt>Stamp on case</dt>
          <dd>{metadata.stamp}</dd>
        </>
      )}
    </dl>
  )
}
