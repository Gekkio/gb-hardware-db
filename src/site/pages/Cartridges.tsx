import * as React from 'react';
import * as R from 'ramda';

import {CartridgeSubmission} from '../../crawler';
import {rejectNil} from '../../util/arrays';

interface GameSubmissions {
  type: string,
  game: string,
  submissions: CartridgeSubmission[],
}

interface Props {
  games: GameSubmissions[],
}

export default function Cartridges({games}: Props) {
  return (
    <article>
      <h2>Game Boy cartridges</h2>
      <h3>Cartridges by game</h3>
      <table>
        <thead>
        <tr>
          <th>Title</th>
          <th>ROM ID</th>
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
  const mappers = R.uniq(rejectNil(submissions.map(({metadata}) => metadata.board.mapper)).map(({type}) => type)).sort();
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/cartridges/${type}/`}>{game}</a>
      </td>
      <td>{type}</td>
      <td>{boardTypes.join(', ')}</td>
      <td>{mappers.join(', ')}</td>
      <td>{submissions.length}</td>
    </tr>
  )
}
