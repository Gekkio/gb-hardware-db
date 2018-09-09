import * as React from 'react'

interface Props {
  children?: React.ReactNode
}

export default function ConsolePageChipTable({ children }: Props) {
  return (
    <table>
      <thead>
        <tr>
          <th />
          <th>Chip</th>
          <th>Type</th>
          <th>Manufacturer</th>
          <th>Date</th>
          <th>Label</th>
        </tr>
      </thead>
      <tbody>{children}</tbody>
    </table>
  )
}
