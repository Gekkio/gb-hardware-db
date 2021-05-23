import * as React from 'react'

import * as config from '../../config'

interface Props {
  pageType: string
}

export default function SiteHeader({ pageType }: Props) {
  return (
    <header className="site-header">
      <div className="site-header__primary">
        <h1 className="site-header__title">
          <a href="/">
            Game Boy hardware database
            <aside>by Gekkio and contributors</aside>
          </a>
        </h1>
        <PrimaryNav pageType={pageType} />
      </div>
      <SecondaryNav pageType={pageType} />
    </header>
  )
}

const models = config.consoles.map((type) => [type.toUpperCase(), type, config.consoleCfgs[type].name])

function isModel(pageType: string, code: string) {
  return pageType === code || pageType === `${code}-console`
}

function isInCartridges(pageType: string): boolean {
  return (
    pageType === 'cartridges' ||
    pageType === 'cartridge' ||
    pageType === 'game' ||
    pageType === 'mapper' ||
    pageType === 'contribute-cartridges'
  )
}

function PrimaryNav({ pageType }: Props) {
  return (
    <nav className="site-primary-nav">
      <ul>
        <li className={isInCartridges(pageType) ? undefined : 'active'}>
          <a href="/">Consoles</a>
        </li>
        <li className={isInCartridges(pageType) ? 'active' : undefined}>
          <a href="/cartridges">Game cartridges</a>
        </li>
      </ul>
    </nav>
  )
}

function SecondaryNav({ pageType }: Props) {
  if (isInCartridges(pageType)) return <nav className="site-secondary-nav" />
  return (
    <nav className="site-secondary-nav">
      <ul>
        {models.map(([model, code, name]) => (
          <li key={code} className={isModel(pageType, code) ? 'active' : undefined}>
            <a href={`/consoles/${code}`}>
              <strong>{model}</strong>
              <span className="name">{name}</span>
            </a>
          </li>
        ))}
      </ul>
    </nav>
  )
}
