import * as React from 'react';

import Home from './pages/Home';
import Contribute from './pages/Contribute';
import ContributeSgb from './pages/ContributeSgb';
import ContributeSgb2 from './pages/ContributeSgb2';
import ContributeOxy from './pages/ContributeOxy';
import Dmg from './pages/Dmg';
import DmgConsole from './pages/DmgConsole';
import Sgb from './pages/Sgb';
import SgbConsole from './pages/SgbConsole';
import Mgb from './pages/Mgb';
import MgbConsole from './pages/MgbConsole';
import Mgl from './pages/Mgl';
import MglConsole from './pages/MglConsole';
import Sgb2 from './pages/Sgb2';
import Sgb2Console from './pages/Sgb2Console';
import Cgb from './pages/Cgb';
import Agb from './pages/Agb';
import AgbConsole from './pages/AgbConsole';
import Ags from './pages/Ags';
import Gbs from './pages/Gbs';
import GbsConsole from './pages/GbsConsole';
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
        <link rel="stylesheet" href="//fonts.googleapis.com/css?family=Lato:400,700" />
        <link rel="stylesheet" href="/static/gbhwdb.css" />
        <script dangerouslySetInnerHTML={{__html: googleAnalytics()}} />
        <script async src="https://www.google-analytics.com/analytics.js" />
      </head>
      <body>
        <SiteHeader pageType={props.pageType} />
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
    case 'contribute-sgb':
      return <ContributeSgb {...props} />
    case 'contribute-sgb2':
      return <ContributeSgb2 {...props} />
    case 'contribute-oxy':
      return <ContributeOxy {...props} />
    case 'dmg':
      return <Dmg {...props} />
    case 'dmg-console':
      return <DmgConsole {...props} />
    case 'sgb':
      return <Sgb {...props} />
    case 'sgb-console':
      return <SgbConsole {...props} />
    case 'mgb':
      return <Mgb {...props} />
    case 'mgb-console':
      return <MgbConsole {...props} />
    case 'mgl':
      return <Mgl {...props} />
    case 'mgl-console':
      return <MglConsole {...props} />
    case 'sgb2':
      return <Sgb2 {...props} />
    case 'sgb2-console':
      return <Sgb2Console {...props} />
    case 'cgb':
      return <Cgb {...props} />
    case 'agb':
      return <Agb {...props} />
    case 'agb-console':
      return <AgbConsole {...props} />
    case 'ags':
      return <Ags {...props} />
    case 'gbs':
      return <Gbs {...props} />
    case 'gbs-console':
      return <GbsConsole {...props} />
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
