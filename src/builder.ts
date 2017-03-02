import 'source-map-support/register';
import * as fs from 'fs-extra';
import * as path from 'path';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';

import Site from './site/Site';
import {Photo, crawlDataDirectory, SgbSubmission, OxySubmission} from './crawler';

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
    {type: 'contribute', title: 'Contribute', props: {}},
    {type: 'consoles', path: ['consoles'], title: 'Game Boy units', props: {}},
    {type: 'sgb', path: ['consoles', 'sgb', 'index'], title: 'Super Game Boy (SGB)', props: {
      submissions: submissions.filter(x => x.type === 'sgb') as SgbSubmission[]
    }},
    {type: 'oxy', path: ['consoles', 'oxy', 'index'], title: 'Game Boy Micro (OXY)', props: {
      submissions: submissions.filter(x => x.type === 'oxy') as OxySubmission[]
    }}
  ]
  submissions.forEach(submission => {
    if (submission.type === 'sgb') {
      pages.push({
        type: 'sgb-console',
        path: ['consoles', 'sgb', submission.slug],
        title: `SGB: ${submission.title}`,
        props: {submission}
      });
    } else if (submission.type === 'oxy') {
      pages.push({
        type: 'oxy-console',
        path: ['consoles', 'oxy', submission.slug],
        title: `OXY: ${submission.title}`,
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
  } else if (submission.type === 'oxy') {
    const photos = R.values(submission.photos).filter(x => !!x) as Photo[];
    if (photos.length === 0) {
      return;
    }

    const targetDirectory = path.resolve('build', 'site', 'static', 'oxy');
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
