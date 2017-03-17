import * as React from 'react';

interface Props {
  pageType: string;
}

export default function SiteHeader({pageType}: Props) {
  return (
    <header className="site-header">
      <h1 className="site-header__title">
        <a href="/">
          Game Boy hardware database
          <aside>by Gekkio and contributors</aside>
        </a>
      </h1>
      <Navigation pageType={pageType} />
    </header>
  )
}

const models = [
  ['DMG', 'Game Boy'],
  ['SGB', 'Super Game Boy'],
  ['MGB', 'Game Boy Pocket'],
  ['MGL', 'Game Boy Light'],
  ['SGB2', 'Super Game Boy 2'],
  ['CGB', 'Game Boy Color'],
  ['AGB', 'Game Boy Advance'],
  ['AGS', 'Game Boy Advance SP'],
  ['GBS', 'Game Boy Player'],
  ['OXY', 'Game Boy Micro'],
].map(([model, name]) => [model, model.toLowerCase(), name]);

function isModel(pageType: string, code: string) {
  return pageType === code || pageType === `${code}-console`
}

function Navigation({pageType}: Props) {
  return (
    <nav className="site-navigation">
      <ul>{
        models.map(([model, code, name]) => (
          <li key={code} className={(isModel(pageType, code)) ? 'active' : undefined}>
            <a href={`/consoles/${code}`}>
              <strong>{model}</strong>
              <span className="name">{name}</span>
            </a>
          </li>
        ))
      }</ul>
    </nav>
  )
}
