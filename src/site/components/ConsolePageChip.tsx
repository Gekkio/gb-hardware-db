import * as R from 'ramda';
import * as React from 'react';

import {Chip} from '../../metadata';
import {formatYearWeek, formatOptional} from '../format';

interface Props {
  designator: string;
  title: string;
  chip?: Chip;
}

export default function ConsolePageChip({designator, title, chip}: Props) {
  return (
    <tr>
      <td>{designator}</td>
      <td>{title}</td>
      <td>{formatOptional(R.identity, chip && chip.type)}</td>
      <td>{formatOptional(formatYearWeek, chip)}</td>
      <td>{formatOptional(R.identity, chip && chip.label)}</td>
    </tr>
  )
}
