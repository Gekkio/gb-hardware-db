import * as React from 'react';
import * as R from 'ramda';

import {CartridgeSubmission} from '../../crawler';
import {rejectNil} from '../../util/arrays';
import {mapperCfgs, MapperId} from '../../config';

interface GameSubmissions {
  type: string,
  game: string,
  submissions: CartridgeSubmission[],
}

interface Props {
  games: GameSubmissions[],
  mappers: MapperId[],
}

export default function Cartridges({games, mappers}: Props) {
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
          <th>Release(s)</th>
          <th>Board type(s)</th>
          <th>Mapper(s)</th>
          <th>Submissions</th>
        </tr>
        </thead>
        <tbody>
        {games.map(game =>
          <Game key={game.type} game={game} />
        )}
        </tbody>
      </table>
    </article>
  )
}

function Game({game: {type, game, submissions}}: {game: GameSubmissions}) {
  const boardTypes = R.uniq(submissions.map(({metadata}) => metadata.board.type)).sort();
  const releases = R.uniq(rejectNil(submissions.map(({metadata}) => metadata.code))).sort();
  const mappers = R.uniq(rejectNil(submissions.map(({metadata}) => metadata.board.mapper)).map(({type}) => type)).sort();
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/cartridges/${type}/`}>{game}</a>
      </td>
      <td>{type}</td>
      <td>{releases.join(', ')}</td>
      <td>{boardTypes.join(', ')}</td>
      <td>{mappers.join(', ')}</td>
      <td>{submissions.length}</td>
    </tr>
  )
}

function Mappers({mappers}: {mappers: MapperId[]}) {
  if (mappers.length === 0) return null;
  return <>
    <h3>Cartridges by mapper</h3>
    <ul>
      {R.sortBy(x => x, mappers).map(mapper =>
        <li key={mapper}>
          <a href={`/cartridges/${mapper}.html`}>{mapperCfgs[mapper].name}</a>
        </li>
      )}
    </ul>
  </>
}
