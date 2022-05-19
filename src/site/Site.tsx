import * as React from 'react'

import SiteFooter from './components/SiteFooter'
import SiteHeader from './components/SiteHeader'
import Cartridge from './pages/Cartridge'
import Game from './pages/Game'
import Cartridges from './pages/Cartridges'
import Mapper from './pages/Mapper'

namespace Site {
  export interface Props {
    pageType: string
    title: string
    pageProps: any
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
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <link rel="manifest" href="/site.webmanifest" />
        <link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5" />
        <meta name="msapplication-TileColor" content="#2b5797" />
        <meta name="theme-color" content="#ffffff" />
        <script dangerouslySetInnerHTML={{ __html: googleAnalytics() }} />
        <script async src="https://www.google-analytics.com/analytics.js" />
      </head>
      <body>
        <SiteHeader pageType={props.pageType} />
        <main className="site-main">
          <div className="site-main__content">{renderPage(props.pageType, props.pageProps)}</div>
        </main>
        <SiteFooter />
      </body>
    </html>
  )
}

function renderPage(type: string, props: any) {
  switch (type) {
    case 'cartridges':
      return <Cartridges {...props} />
    case 'cartridge':
      return <Cartridge {...props} />
    case 'game':
      return <Game {...props} />
    case 'mapper':
      return <Mapper {...props} />
    default:
      return null
  }
}

function googleAnalytics() {
  return `window.ga=window.ga||function(){(ga.q=ga.q||[]).push(arguments)};ga.l=+new Date;
  ga('create', 'UA-37123121-2', 'auto');
  ga('send', 'pageview');`
}
