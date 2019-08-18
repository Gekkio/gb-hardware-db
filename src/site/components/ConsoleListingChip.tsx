import * as classnames from 'classnames'
import * as R from 'ramda'
import * as React from 'react'

import { Chip } from '../../metadata'
import * as format from '../format'

interface Props {
  chip?: Chip | null
  hideType?: boolean
}

export default function ConsoleListingChip({ chip, hideType }: Props) {
  if (chip === undefined) {
    return <td />
  } else if (chip === null) {
    return <td>-</td>
  }
  const classes = classnames('console-listing-chip', { 'console-listing-chip--outlier': !!chip.outlier })
  return (
    <td className={classes}>
      {!hideType && <div>{format.optional<string>(R.identity, chip.type)}</div>}
      <div>{format.short.calendar(chip)}</div>
      <div>{format.optional(format.manufacturer, chip.manufacturer)}</div>
    </td>
  )
}
