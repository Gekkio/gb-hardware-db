import * as React from 'react';
import * as R from 'ramda';

import {CartridgeSubmission, Photo} from '../../crawler';
import * as format from '../format';
import {gameCfgs, GameConfig, gameLayouts, mapperCfgs, MapperId} from '../../config';
import ConsoleListingChip from '../components/ConsoleListingChip';

interface Props {
  mapper: MapperId,
  submissions: CartridgeSubmission[];
}

export default function Mapper({mapper, submissions}: Props) {
  const cfg = mapperCfgs[mapper]
  const groupedSubmissions = R.groupBy(({type}) => type, submissions);
  const games = R.sortBy(type => gameCfgs[type].name, Object.keys(groupedSubmissions));
  const sortedSubmissions: CartridgeSubmission[] = [];
  for (const game of games) {
    sortedSubmissions.push(...groupedSubmissions[game])
  }
  return (
    <article>
      <h2>{`Cartridges by mapper: ${cfg.name}`}</h2>
      <table>
        <thead>
        <tr>
          <th>Entry</th>
          <th>ROM ID</th>
          <th>Release</th>
          <th>Board</th>
          <th>Mapper</th>
          <th>Photos</th>
        </tr>
        </thead>
        <tbody>
        {sortedSubmissions.map(submission =>
          <Submission key={submission.slug} type={submission.type} cfg={gameCfgs[submission.type]} submission={submission} />
        )}
        </tbody>
      </table>
    </article>
  )
}

function Submission({type, cfg, submission: {contributor, slug, title, metadata, photos}}: {type: string, cfg: GameConfig, submission: CartridgeSubmission}) {
  const layout = gameLayouts[cfg.layout];
  return (
    <tr>
      <td className="submission-list-item">
        <a className="submission-list-item__link" href={`/cartridges/${type}/${slug}.html`}>
          <div className="submission-list-item__photo">
            {photos.front
              ? <img
                src={`/static/${type}/${slug}_thumbnail_80.jpg`}
                srcSet={`/static/${type}/${slug}_thumbnail_50.jpg 50w, /static/${type}/${slug}_thumbnail_80.jpg 80w`}
                sizes="(min-width: 1000px) 80px, 50px"
                role="presentation" />
              : null
            }
          </div>
          <div className="submission-list-item__id">
            <div className="submission-list-item__title">{cfg.name}</div>
            <aside>{title}</aside>
            <aside className="submission-list-item__contributor">{contributor}</aside>
          </div>
        </a>
      </td>
      <td>{type}</td>
      <td>{format.optional(x => x, metadata.code)}</td>
      <td>
        <div>{metadata.board.type}</div>
        <div>{format.short.calendar(metadata.board)}</div>
      </td>
      <ConsoleListingChip chip={metadata.board.mapper} />
      <td>
        {renderPhoto(type, slug, 'Front', photos.front)}
        {renderPhoto(type, slug, 'Back', photos.back)}
        {renderPhoto(type, slug, 'PCB front', photos.pcbFront)}
        {renderPhoto(type, slug, 'PCB back', photos.pcbBack)}
      </td>
    </tr>
  )
}

function renderPhoto(type: string, slug: string, label: string, photo: Photo | undefined) {
  if (!photo) {
    return null;
  }
  return (
    <div>
      <a href={`/static/${type}/${slug}_${photo.name}`}>{label}</a>
    </div>
  )
}
