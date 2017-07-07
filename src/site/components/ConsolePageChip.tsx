import * as R from 'ramda';
import * as React from 'react';

import {Chip} from '../../metadata';
import * as format from '../format';
import * as classnames from 'classnames';

interface Props {
  designator: string;
  title: string;
  chip?: Chip | null;
}

export default function ConsolePageChip({designator, title, chip}: Props) {
  const classes = classnames('console-page-chip', {'console-page-chip--outlier': !!(chip && chip.outlier)});
  return (
    <tr className={classes}>
      <td>{designator}</td>
      <td>{title}</td>
      <td>{format.optional<string>(R.identity, chip && chip.type)}</td>
      <td>{format.optional<string>(format.manufacturer, chip && chip.manufacturer)}</td>
      <td>{format.optional<Chip>(format.calendar, chip)}</td>
      <td>{format.optional<string>(R.identity, chip && chip.label)}</td>
    </tr>
  )
}
