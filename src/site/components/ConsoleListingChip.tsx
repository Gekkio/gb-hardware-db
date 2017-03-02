import * as classnames from 'classnames';
import * as R from 'ramda';
import * as React from 'react';

import {Chip} from '../../metadata';
import {formatShortYearWeek, formatOptional} from '../format';

interface Props {
  chip?: Chip;
}

export default function ConsoleListingChip({chip}: Props) {
  if (chip === undefined) {
    return <td>????</td>
  } else if (chip === null) {
    return <td>-</td>
  }
  const classes = classnames('console-listing-chip', {'console-listing-chip--outlier': !!chip.outlier});
  return (
    <td className={classes}>
      <div>{formatOptional(R.identity, chip.type)}</div>
      <div>{formatShortYearWeek(chip)}</div>
    </td>
  )
}
