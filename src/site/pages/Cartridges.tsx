import * as React from 'react'
import * as R from 'ramda'

import { CartridgeSubmission } from '../../crawler'
import { rejectNil } from '../../util/arrays'
import { mapperCfgs, MapperId, GameConfig } from '../../config'

interface GameSubmissions {
  type: string
  cfg: GameConfig
  submissions: CartridgeSubmission[]
}

interface Props {
  games: GameSubmissions[]
  mappers: MapperId[]
}

export default function Cartridges({ games, mappers }: Props) {
  return (
    <article>
      <h2>Game Boy cartridges</h2>
      <Mappers mappers={mappers} />
      <h3>Cartridges by game</h3>
      <table>
        <thead>
          <tr>
            <th>Title</th>
            <th>ROM ID</th>
            <th>Year(s)</th>
            <th>Release(s)</th>
            <th>Board type(s)</th>
            <th>Mapper(s)</th>
            <th>Submissions</th>
          </tr>
        </thead>
        <tbody className="divider">
          <tr>
            <th colSpan={7}>Game Boy</th>
          </tr>
        </tbody>
        <tbody>
          {games
            .filter(({ cfg }) => cfg.platform === 'gb')
            .map(game => (
              <Game key={game.type} game={game} />
            ))}
        </tbody>
        <tbody className="divider">
          <tr>
            <th colSpan={7}>Game Boy Color</th>
          </tr>
        </tbody>
        <tbody>
          {games
            .filter(({ cfg }) => cfg.platform === 'gbc')
            .map(game => (
              <Game key={game.type} game={game} />
            ))}
        </tbody>
      </table>
      <h3>Data dumps</h3>
      <a href="/static/export/cartridges.csv">UTF-8 encoded CSV</a>
    </article>
  )
}

function multiline(lines: string[]) {
  return R.uniq(lines)
    .sort()
    .map((line, idx) => (
      <span key={idx}>
        {line}
        <br />
      </span>
    ))
}

function Game({ game: { type, cfg, submissions } }: { game: GameSubmissions }) {
  const boardTypes = submissions.map(({ metadata }) => metadata.board.type)
  const years = rejectNil(submissions.map(({ metadata }) => metadata.board.year)).map(String)
  const releases = rejectNil(submissions.map(({ metadata }) => metadata.code))
  const mappers = rejectNil(
    submissions.map(({ metadata }) => metadata.board.mapper).map(mapper => mapper && mapper.kind)
  )
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/cartridges/${type}/`}>
          {cfg.name}
        </a>
      </td>
      <td>{type}</td>
      <td>{multiline(years)}</td>
      <td>{multiline(releases)}</td>
      <td>{multiline(boardTypes)}</td>
      <td>{multiline(mappers)}</td>
      <td>{submissions.length}</td>
    </tr>
  )
}

function Mappers({ mappers }: { mappers: MapperId[] }) {
  if (mappers.length === 0) return null
  return (
    <>
      <h3>Cartridges by mapper</h3>
      <ul>
        {R.sortBy(x => x, mappers).map(mapper => (
          <li key={mapper}>
            <a href={`/cartridges/${mapper}.html`}>{mapperCfgs[mapper].name}</a>
          </li>
        ))}
      </ul>
    </>
  )
}
