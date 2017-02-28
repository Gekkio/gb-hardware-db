import * as React from 'react';

import Home from './pages/Home';
import Contribute from './pages/Contribute';
import SgbUnit from './pages/SgbUnit';
import SiteFooter from './components/SiteFooter';
import SiteHeader from './components/SiteHeader';

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
        <SiteHeader />
        <main className="site-main">
          <div className="site-main__content">
            {renderPage(props.pageType, props.pageProps)}
          </div>
        </main>
        <SiteFooter />
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

