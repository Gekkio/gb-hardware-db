import * as React from 'react';

import Home from './pages/Home';
import Contribute from './pages/Contribute';
import Sgb from './pages/Sgb';
import SgbConsole from './pages/SgbConsole';
import Sgb2 from './pages/Sgb2';
import Sgb2Console from './pages/Sgb2Console';
import Oxy from './pages/Oxy';
import OxyConsole from './pages/OxyConsole';
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
        <script dangerouslySetInnerHTML={{__html: googleAnalytics()}} />
        <script async src="https://www.google-analytics.com/analytics.js" />
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
    case 'sgb':
      return <Sgb {...props} />
    case 'sgb-console':
      return <SgbConsole {...props} />
    case 'sgb2':
      return <Sgb2 {...props} />
    case 'sgb2-console':
      return <Sgb2Console {...props} />
    case 'oxy':
      return <Oxy {...props} />
    case 'oxy-console':
      return <OxyConsole {...props} />
    default:
      return null;
  }
}

function googleAnalytics() {
  return `window.ga=window.ga||function(){(ga.q=ga.q||[]).push(arguments)};ga.l=+new Date;
  ga('create', 'UA-37123121-2', 'auto');
  ga('send', 'pageview');`;
}
