import 'source-map-support/register';
import * as fs from 'fs-extra';
import * as path from 'path';
import * as Joi from 'joi';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';

import Site from './Site';
import {Photo, crawlDataDirectory} from './crawler';

interface PageDeclaration {
  type: string;
  path?: string[];
  title: string;
  props: any;
}

const submissions = crawlDataDirectory('data');

function resolvePages(): PageDeclaration[] {
  const pages: PageDeclaration[] = [
    {type: 'index', title: 'Home', props: {}},
    {type: 'contribute', title: 'Contribute', props: {}}
  ]
  submissions.forEach(submission => {
    if (submission.type === 'sgb') {
      pages.push({
        type: 'sgb-unit',
        path: ['sgb', submission.slug],
        title: submission.title,
        props: {submission}
      });
    }
  })
  return pages;
}

submissions.forEach(submission => {
  if (submission.type === 'sgb') {
    const photos = R.values(submission.photos).filter(x => !!x) as Photo[];
    if (photos.length === 0) {
      return;
    }

    const targetDirectory = path.resolve('build', 'site', 'static', 'sgb');
    fs.ensureDirSync(targetDirectory);

    photos.forEach(photo => {
      const target = path.resolve(targetDirectory, `${submission.slug}_${photo.name}`);
      fs.copySync(photo.path, target, {preserveTimestamps: true});
      console.log(`Copied ${target}`);
    })
  }
})

resolvePages().forEach(page => {
  const props = {
    pageType: page.type,
    title: `${page.title} - Game Boy hardware database`,
    pageProps: page.props
  };
  const markup = ReactDOMServer.renderToStaticMarkup(React.createElement(Site, props));
  const html = `<!DOCTYPE html>\n${markup}`

  const directories = R.init(page.path || []);
  const targetDirectory = path.resolve('build', 'site', ...directories);

  const filename = R.last(page.path || []) || page.type;
  const target = path.resolve(targetDirectory, `${filename}.html`);
  fs.outputFileSync(target, html);
  console.log(`Wrote ${target}`);
});
