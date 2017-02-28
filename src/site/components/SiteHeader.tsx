import * as React from 'react';

export default function SiteHeader() {
  return (
    <header className="site-header">
      <h1 className="site-header__title">
        <a href="/">
          Game Boy hardware database
          <aside>by Gekkio and contributors</aside>
        </a>
      </h1>
      <Navigation />
    </header>
  )
}

function Navigation() {
  return (
    <nav className="site-navigation">
    </nav>
  )
}
