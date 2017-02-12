import * as fs from 'fs';
import * as path from 'path';
import * as Joi from 'joi';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';

import Site from './Site';

type PageDeclaration = [string, string, () => any];

const pages: PageDeclaration[] = [
  ['index', 'Home', () => ({})],
  ['contribute', 'Contribute', () => ({})]
]

pages.forEach(([id, title, createPageProps]) => {
  const props = {
    pageId: id,
    title: `${title} - Game Boy hardware database`,
    pageProps: createPageProps()
  };
  const markup = ReactDOMServer.renderToStaticMarkup(React.createElement(Site, props));
  const html = `<!DOCTYPE html>\n${markup}`
  const targetDirectory = path.resolve('build', 'site');

  try {
    fs.mkdirSync(targetDirectory);
  } catch(e) {
    if (e.code !== 'EEXIST') {
      throw e;
    }
  }
  const target = path.resolve(targetDirectory, `${id}.html`);
  fs.writeFileSync(target, html);
});
