import 'source-map-support/register';
import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';
import * as fs from 'fs';
import * as path from 'path';
import * as winston from 'winston';

import Site from '../site/Site';
import {
  crawlDataDirectory, DmgSubmission, SgbSubmission, MgbSubmission, MglSubmission, Sgb2Submission, CgbSubmission, AgbSubmission,
  AgsSubmission, GbsSubmission, OxySubmission
} from '../crawler';
import processPhotos from './processPhotos';
import * as files from '../util/files';

interface PageDeclaration {
  type: string;
  path?: string[];
  title: string;
  props: any;
}

const submissions = crawlDataDirectory('data');

function resolvePages(): PageDeclaration[] {
  const pages: PageDeclaration[] = [
    {type: 'index', title: 'Home', props: {
      content: fs.readFileSync('content/home.markdown', {encoding: 'utf-8'})
    }},
    {type: 'contribute', path: ['contribute', 'index'], title: 'Contribute', props: {}},
    {type: 'contribute-sgb', path: ['contribute', 'sgb'], title: 'Super Game Boy (SGB) contribution instructions', props: {}},
    {type: 'contribute-sgb2', path: ['contribute', 'sgb2'], title: 'Super Game Boy 2 (SGB2) contribution instructions', props: {}},
    {type: 'contribute-oxy', path: ['contribute', 'oxy'], title: 'Game Boy Micro (OXY) contribution instructions', props: {}},
    {type: 'consoles', path: ['consoles'], title: 'Game Boy units', props: {}},
    {type: 'dmg', path: ['consoles', 'dmg', 'index'], title: 'Game Boy (DMG)', props: {
      submissions: submissions.filter(x => x.type === 'dmg') as DmgSubmission[]
    }},
    {type: 'sgb', path: ['consoles', 'sgb', 'index'], title: 'Super Game Boy (SGB)', props: {
      submissions: submissions.filter(x => x.type === 'sgb') as SgbSubmission[]
    }},
    {type: 'mgb', path: ['consoles', 'mgb', 'index'], title: 'Game Boy Pocket (MGB)', props: {
      submissions: submissions.filter(x => x.type === 'mgb') as MgbSubmission[]
    }},
    {type: 'mgl', path: ['consoles', 'mgl', 'index'], title: 'Game Boy Light (MGL)', props: {
      submissions: submissions.filter(x => x.type === 'mgl') as MglSubmission[]
    }},
    {type: 'sgb2', path: ['consoles', 'sgb2', 'index'], title: 'Super Game Boy 2 (SGB2)', props: {
      submissions: submissions.filter(x => x.type === 'sgb2') as Sgb2Submission[]
    }},
    {type: 'cgb', path: ['consoles', 'cgb', 'index'], title: 'Game Boy Color (CGB)', props: {
      submissions: submissions.filter(x => x.type === 'cgb') as CgbSubmission[]
    }},
    {type: 'agb', path: ['consoles', 'agb', 'index'], title: 'Game Boy Advance (AGB)', props: {
      submissions: submissions.filter(x => x.type === 'agb') as AgbSubmission[]
    }},
    {type: 'ags', path: ['consoles', 'ags', 'index'], title: 'Game Boy Advance SP (AGS)', props: {
      submissions: submissions.filter(x => x.type === 'ags') as AgsSubmission[]
    }},
    {type: 'gbs', path: ['consoles', 'gbs', 'index'], title: 'Game Boy Player (GBS)', props: {
      submissions: submissions.filter(x => x.type === 'gbs') as GbsSubmission[]
    }},
    {type: 'oxy', path: ['consoles', 'oxy', 'index'], title: 'Game Boy Micro (OXY)', props: {
      submissions: submissions.filter(x => x.type === 'oxy') as OxySubmission[]
    }}
  ]
  submissions.forEach(submission => {
    if (submission.type === 'dmg') {
      pages.push({
        type: 'dmg-console',
        path: ['consoles', 'dmg', submission.slug],
        title: `DMG: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'sgb') {
      pages.push({
        type: 'sgb-console',
        path: ['consoles', 'sgb', submission.slug],
        title: `SGB: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'sgb2') {
      pages.push({
        type: 'sgb2-console',
        path: ['consoles', 'sgb2', submission.slug],
        title: `SGB2: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'agb') {
      pages.push({
        type: 'agb-console',
        path: ['consoles', 'agb', submission.slug],
        title: `AGB: ${submission.title}`,
        props: {submission}
      });
    } else if (submission.type === 'gbs') {
      pages.push({
        type: 'gbs-console',
        path: ['consoles', 'gbs', submission.slug],
        title: `GBS: ${submission.title}`,
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

    return files.outputFile(target, html)
      .tap(() => winston.debug(`Wrote HTML file ${target}`))
  }));
}

Bluebird.all([photosPromise, processPages()])
  .then(() => {
    winston.info('Site generation finished :)');
    return null
  })
  .catch(e => {
    console.error(e.stack || e);
  })
