import * as React from 'react';
import * as humanDate from 'human-date';

import Home from './pages/Home';
import Contribute from "./pages/Contribute";
import SgbUnit from './pages/SgbUnit';

namespace Site {
  export interface Props {
    pageType: string;
    title: string;
    pageProps: any;
  }
}

export default function Site(props: Site.Props) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta httpEquiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>{props.title}</title>
        <link rel="stylesheet" href="//fonts.googleapis.com/css?family=Lora:400,700" />
        <link rel="stylesheet" href="/static/gbhwdb.css" />
      </head>
      <body>
        <header id="site-header">
          <h1>
            <a href="/">
              Game Boy hardware database
              <aside>by Gekkio and contributors</aside>
            </a>
          </h1>
          <Navigation />
        </header>
        <main id="site-content">
          <div className="content">
            {renderPage(props.pageType, props.pageProps)}
          </div>
        </main>
        <footer id="site-footer">
          <div className="content">
            <License />
            <Stats />
          </div>
        </footer>
      </body>
    </html>
  );
}

function renderPage(type: string, props: any) {
  switch (type) {
    case 'index':
      return <Home {...props} />
    case 'contribute':
      return <Contribute {...props} />
    case 'sgb-unit':
      return <SgbUnit {...props} />
    default:
      return null;
  }
}

function Navigation() {
  return (
    <nav>
    </nav>
  )
}

function License() {
  return (
    <aside id="site-license">
      <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/" className="badge">
        <img alt="Creative Commons License" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"
             width="88" height="31" />
      </a>
      <p>
        The data and photos on this site are licensed under
        the <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons
      Attribution-ShareAlike 4.0 International License</a>.
      </p>
      <p>
        The <a href="https://github.com/Gekkio/gb-hardware-db">site source code</a> is licensed under the MIT license.
      </p>
    </aside>
  );
}

function Stats() {
  return (
    <aside id="site-stats">
      {`Last updated: ${humanDate.prettyPrint()}`}
      <br />
      <a href="/contribute.html">Want to contribute?</a>
    </aside>
  );
}
