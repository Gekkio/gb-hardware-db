import classnames from 'classnames'
import React from 'react'

import { Chip } from '../../metadata'
import * as format from '../format'

interface Props {
  designator: string
  title: string
  chip?: Chip | null
}

export default function ConsolePageChip({ designator, title, chip }: Props) {
  const classes = classnames('console-page-chip', { 'console-page-chip--outlier': !!(chip && chip.outlier) })
  return (
    <tr className={classes}>
      <td>{designator}</td>
      <td>{title}</td>
      <td>{chip && chip.kind}</td>
      <td>{chip && chip.manufacturer}</td>
      <td>{chip && format.calendar(chip)}</td>
      <td>{chip && chip.label}</td>
    </tr>
  )
}
