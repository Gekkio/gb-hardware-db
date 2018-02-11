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
    {type: 'dmg', path: ['consoles', 'dmg', 'index'], title: 'Game Boy (DMG)', props: {
      submissions: groupedSubmissions.dmg,
    }},
    {type: 'sgb', path: ['consoles', 'sgb', 'index'], title: 'Super Game Boy (SGB)', props: {
      submissions: groupedSubmissions.sgb,
    }},
    {type: 'mgb', path: ['consoles', 'mgb', 'index'], title: 'Game Boy Pocket (MGB)', props: {
      submissions: groupedSubmissions.mgb,
    }},
    {type: 'mgl', path: ['consoles', 'mgl', 'index'], title: 'Game Boy Light (MGL)', props: {
      submissions: groupedSubmissions.mgl,
    }},
    {type: 'sgb2', path: ['consoles', 'sgb2', 'index'], title: 'Super Game Boy 2 (SGB2)', props: {
      submissions: groupedSubmissions.sgb2,
    }},
    {type: 'cgb', path: ['consoles', 'cgb', 'index'], title: 'Game Boy Color (CGB)', props: {
      submissions: groupedSubmissions.cgb,
    }},
    {type: 'agb', path: ['consoles', 'agb', 'index'], title: 'Game Boy Advance (AGB)', props: {
      submissions: groupedSubmissions.agb,
    }},
    {type: 'ags', path: ['consoles', 'ags', 'index'], title: 'Game Boy Advance SP (AGS)', props: {
      submissions: groupedSubmissions.ags,
    }},
    {type: 'gbs', path: ['consoles', 'gbs', 'index'], title: 'Game Boy Player (GBS)', props: {
      submissions: groupedSubmissions.gbs,
    }},
    {type: 'oxy', path: ['consoles', 'oxy', 'index'], title: 'Game Boy Micro (OXY)', props: {
      submissions: groupedSubmissions.oxy,
    }}
  ];
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
    } else if (submission.type === 'mgb') {
      pages.push({
        type: 'mgb-console',
        path: ['consoles', 'mgb', submission.slug],
        title: `MGB: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'mgl') {
      pages.push({
        type: 'mgl-console',
        path: ['consoles', 'mgl', submission.slug],
        title: `MGL: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'sgb2') {
      pages.push({
        type: 'sgb2-console',
        path: ['consoles', 'sgb2', submission.slug],
        title: `SGB2: ${submission.title} [${submission.contributor}]`,
        props: {submission}
      });
    } else if (submission.type === 'cgb') {
      pages.push({
        type: 'cgb-console',
        path: ['consoles', 'cgb', submission.slug],
        title: `CGB: ${submission.title}`,
        props: {submission}
      });
    } else if (submission.type === 'agb') {
      pages.push({
        type: 'agb-console',
        path: ['consoles', 'agb', submission.slug],
        title: `AGB: ${submission.title}`,
        props: {submission}
      });
    } else if (submission.type === 'ags') {
      pages.push({
        type: 'ags-console',
        path: ['consoles', 'ags', submission.slug],
        title: `AGS: ${submission.title}`,
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
  });

  await Promise.all([
    Bluebird.map(pages, processPage, {concurrency: 16}),
    Bluebird.map(submissions, processPhotos, {concurrency: 2}),
  ]);

  await([
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
