import 'source-map-support/register';
import * as Bluebird from 'bluebird';
import * as R from 'ramda';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';
import * as fs from 'fs-extra';
import * as path from 'path';
import * as process from 'process';
import * as winston from 'winston';

import Site from '../site/Site';
import {
  AgbSubmission, AgsSubmission, CgbSubmission, crawlDataDirectory, DmgSubmission, GbsSubmission, MgbSubmission,
  MglSubmission, OxySubmission, Sgb2Submission, SgbSubmission, Submission
} from '../crawler';
import processPhotos from './processPhotos';
import {
  AGB_CSV_COLUMNS, AGS_CSV_COLUMNS, CGB_CSV_COLUMNS, CsvColumn, DMG_CSV_COLUMNS, GBS_CSV_COLUMNS, generateCsv,
  MGB_CSV_COLUMNS, MGL_CSV_COLUMNS, OXY_CSV_COLUMNS, SGB2_CSV_COLUMNS, SGB_CSV_COLUMNS
} from './csvTransform';
import * as config from '../config';

interface PageDeclaration {
  type: string;
  path?: string[];
  title: string;
  props: any;
}

interface GroupedSubmissions {
  dmg: DmgSubmission[],
  sgb: SgbSubmission[],
  mgb: MgbSubmission[],
  mgl: MglSubmission[],
  sgb2: Sgb2Submission[],
  cgb: CgbSubmission[],
  agb: AgbSubmission[],
  ags: AgsSubmission[],
  gbs: GbsSubmission[],
  oxy: OxySubmission[],
}

function groupSubmissions(submissions: Submission[]): GroupedSubmissions {
  return R.groupBy(submission => submission.type, submissions) as any
}

async function main(): Promise<void> {
  const submissions = await crawlDataDirectory('data');

  const groupedSubmissions = groupSubmissions(submissions);

  const pages: PageDeclaration[] = [
    {type: 'index', title: 'Home', props: {
      content: await fs.readFile('content/home.markdown', {encoding: 'utf-8'})
    }},
    {type: 'contribute', path: ['contribute', 'index'], title: 'Contribute', props: {}},
    {type: 'contribute-sgb', path: ['contribute', 'sgb'], title: 'Super Game Boy (SGB) contribution instructions', props: {}},
    {type: 'contribute-sgb2', path: ['contribute', 'sgb2'], title: 'Super Game Boy 2 (SGB2) contribution instructions', props: {}},
    {type: 'contribute-oxy', path: ['contribute', 'oxy'], title: 'Game Boy Micro (OXY) contribution instructions', props: {}},
    {type: 'consoles', path: ['consoles'], title: 'Game Boy units', props: {}},
    ...config.consoles.map(type => {
      const cfg = config.consoleCfgs[type]
      return {type, path: ['consoles', type, 'index'], title: `${cfg.name} (${type.toUpperCase()})`, props: {
        submissions: groupedSubmissions[type],
      }}
    })
  ];
  submissions.forEach(submission => {
    const {type, slug, title, contributor} = submission
    const cfg = config.consoleCfgs[type]
    pages.push({
      type: `${type}-console`,
      path: ['consoles', type, slug],
      title: `${type.toUpperCase()}: ${title} [${contributor}]`,
      props: {submission}
    });
  });

  await Promise.all([
    Bluebird.map(pages, processPage, {concurrency: 16}),
    Bluebird.map(submissions, processPhotos, {concurrency: 2}),
  ]);

  await Promise.all([
    processCsv('dmg', DMG_CSV_COLUMNS, groupedSubmissions.dmg),
    processCsv('sgb', SGB_CSV_COLUMNS, groupedSubmissions.sgb),
    processCsv('mgb', MGB_CSV_COLUMNS, groupedSubmissions.mgb),
    processCsv('mgl', MGL_CSV_COLUMNS, groupedSubmissions.mgl),
    processCsv('sgb2', SGB2_CSV_COLUMNS, groupedSubmissions.sgb2),
    processCsv('cgb', CGB_CSV_COLUMNS, groupedSubmissions.cgb),
    processCsv('agb', AGB_CSV_COLUMNS, groupedSubmissions.agb),
    processCsv('ags', AGS_CSV_COLUMNS, groupedSubmissions.ags),
    processCsv('gbs', GBS_CSV_COLUMNS, groupedSubmissions.gbs),
    processCsv('oxy', OXY_CSV_COLUMNS, groupedSubmissions.oxy),
  ]);
  winston.info('Site generation finished :)');
}

async function processCsv<T, K extends keyof GroupedSubmissions>(
  key: K,
  columns: CsvColumn<T>[],
  rows: T[],
): Promise<void> {
  return generateCsv(columns, rows, path.resolve('build', 'site', 'static', `${key}.csv`))
}

async function processPage(page: PageDeclaration): Promise<void> {
  const props = {
    pageType: page.type,
    title: `${page.title} - Game Boy hardware database`,
    pageProps: page.props
  };
  const markup = ReactDOMServer.renderToStaticMarkup(React.createElement(Site, props));
  const html = `<!DOCTYPE html>\n${markup}`;

  const directories = R.init(page.path || []);
  const targetDirectory = path.resolve('build', 'site', ...directories);

  const filename = R.last(page.path || []) || page.type;
  const target = path.resolve(targetDirectory, `${filename}.html`);

  await fs.outputFile(target, html);
  winston.debug(`Wrote HTML file ${target}`);
}

main()
  .then(() => null)
  .catch(e => {
    if (e.isJoi) {
      console.error(e.annotate());
    } else {
      console.error(e.stack || e);
    }
    process.exit(1);
  });
