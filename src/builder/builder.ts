import 'source-map-support/register';
import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';
import * as fs from 'fs-extra';
import * as path from 'path';

import Site from '../site/Site';
import {crawlDataDirectory, SgbSubmission, OxySubmission, Sgb2Submission} from '../crawler';
import processPhotos from './processPhotos';

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
    {type: 'contribute', path: ['contribute', 'index'], title: 'Contribute', props: {}},
    {type: 'contribute-sgb', path: ['contribute', 'sgb'], title: 'Super Game Boy (SGB) contribution instructions', props: {}},
    {type: 'contribute-sgb2', path: ['contribute', 'sgb2'], title: 'Super Game Boy 2 (SGB2) contribution instructions', props: {}},
    {type: 'contribute-oxy', path: ['contribute', 'oxy'], title: 'Game Boy Micro (OXY) contribution instructions', props: {}},
    {type: 'consoles', path: ['consoles'], title: 'Game Boy units', props: {}},
    {type: 'sgb', path: ['consoles', 'sgb', 'index'], title: 'Super Game Boy (SGB)', props: {
      submissions: submissions.filter(x => x.type === 'sgb') as SgbSubmission[]
    }},
    {type: 'sgb2', path: ['consoles', 'sgb2', 'index'], title: 'Super Game Boy 2 (SGB2)', props: {
      submissions: submissions.filter(x => x.type === 'sgb2') as Sgb2Submission[]
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
    } else if (submission.type === 'sgb2') {
      pages.push({
        type: 'sgb2-console',
        path: ['consoles', 'sgb2', submission.slug],
        title: `SGB2: ${submission.title}`,
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

const outputFile: (file: string, data: any) => Bluebird<{}> = Bluebird.promisify(fs.outputFile) as any;

const photosPromise = Bluebird.all(submissions.map(processPhotos))

function processPages(): Bluebird<any> {
  return Bluebird.all(resolvePages().map(page => {
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

    return outputFile(target, html)
      .tap(() => console.log(`Wrote ${target}`));
  }));
}

Bluebird.all([photosPromise, processPages()])
  .then(() => {
    console.info('All done :)');
    return null
  })
  .catch(e => {
    console.error(e.stack || e);
  })
