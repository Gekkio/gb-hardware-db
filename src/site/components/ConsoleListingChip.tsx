import * as classnames from 'classnames'
import * as React from 'react'

import { Chip } from '../../metadata'
import * as format from '../format'

interface Props {
  chip?: Chip | null
  hideType?: boolean
}

export default function ConsoleListingChip({ chip, hideType }: Props) {
  if (!chip) {
    return <td />
  }
  const classes = classnames('console-listing-chip', { 'console-listing-chip--outlier': !!chip.outlier })
  return (
    <td className={classes}>
      {!hideType && <div>{chip.kind}</div>}
      <div>{chip.rom_code}</div>
      <div>{format.short.calendar(chip)}</div>
      <div>{chip.manufacturer}</div>
    </td>
  )
}
