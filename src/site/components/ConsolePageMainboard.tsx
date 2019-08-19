import * as format from '../format'
import * as React from 'react'
import { ConsoleSubmission } from '../../crawler'

export default function ConsolePageMainboard({ submission }: { submission: ConsoleSubmission }) {
  const {
    metadata: { mainboard },
  } = submission
  return (
    <dl>
      <dt>Board type</dt>
      <dd>{mainboard.type}</dd>
      {'year' in mainboard && mainboard.year && (
        <>
          <dt>Manufacture date</dt>
          <dd>{format.calendar(mainboard)}</dd>
        </>
      )}
      {'number_pair' in mainboard && (
        <>
          <dt>Number pair on board</dt>
          <dd>{mainboard.number_pair}</dd>
        </>
      )}
      {'stamp' in mainboard && (
        <>
          <dt>Stamp on board</dt>
          <dd>{mainboard.stamp}</dd>
        </>
      )}
      {'stamp_front' in mainboard && (
        <>
          <dt>Secondary stamp on board (front)</dt>
          <dd>{mainboard.stamp_front}</dd>
        </>
      )}
      {'stamp_back' in mainboard && (
        <>
          <dt>Secondary stamp on board (back)</dt>
          <dd>{mainboard.stamp_back}</dd>
        </>
      )}
      {'circled_letters' in mainboard && (
        <>
          <dt>Circled letter(s) on board</dt>
          <dd>{mainboard.circled_letters}</dd>
        </>
      )}
      {'letter_at_top_right' in mainboard && (
        <>
          <dt>Letter at top right</dt>
          <dd>{mainboard.letter_at_top_right}</dd>
        </>
      )}
      {'extra_label' in mainboard && (
        <>
          <dt>Extra label</dt>
          <dd>{mainboard.extra_label}</dd>
        </>
      )}
    </dl>
  )
}
