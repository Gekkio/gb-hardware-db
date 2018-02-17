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
  AgbSubmission, AgsSubmission, CartridgeSubmission, CgbSubmission, crawlCartridges, crawlConsoles, DmgSubmission,
  GbsSubmission, MgbSubmission, MglSubmission, OxySubmission, Sgb2Submission, SgbSubmission
} from '../crawler';
import {
  AGB_CSV_COLUMNS, AGS_CSV_COLUMNS, CGB_CSV_COLUMNS, CsvColumn, DMG_CSV_COLUMNS, GBS_CSV_COLUMNS, generateCsv,
  MGB_CSV_COLUMNS, MGL_CSV_COLUMNS, OXY_CSV_COLUMNS, SGB2_CSV_COLUMNS, SGB_CSV_COLUMNS
} from './csvTransform';
import * as config from '../config';
import processPhotos from './processPhotos';

interface PageDeclaration {
  type: string;
  path?: string[];
  title: string;
  props: any;
}

interface GroupedConsoleSubmissions {
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

async function main(): Promise<void> {
  const [consoleSubmissions, cartridgeSubmissions] = await Promise.all([
    crawlConsoles('data/consoles'),
    crawlCartridges('data/cartridges'),
  ]);

  const groupedConsoles: GroupedConsoleSubmissions = R.groupBy(({type}) => type, consoleSubmissions) as any
  const groupedCartridges: Record<string, CartridgeSubmission[]> = R.groupBy(({type}) => type, cartridgeSubmissions)

  const pages: PageDeclaration[] = [
    {type: 'index', title: 'Home', props: {
      content: await fs.readFile('content/home.markdown', {encoding: 'utf-8'})
    }},
    {type: 'contribute', path: ['contribute', 'index'], title: 'Contribute', props: {}},
    {type: 'contribute-sgb', path: ['contribute', 'sgb'], title: 'Super Game Boy (SGB) contribution instructions', props: {}},
    {type: 'contribute-sgb2', path: ['contribute', 'sgb2'], title: 'Super Game Boy 2 (SGB2) contribution instructions', props: {}},
    {type: 'contribute-oxy', path: ['contribute', 'oxy'], title: 'Game Boy Micro (OXY) contribution instructions', props: {}},
    {type: 'consoles', path: ['consoles', 'index'], title: 'Game Boy consoles', props: {}},
    ...config.consoles.map(type => {
      const cfg = config.consoleCfgs[type];
      return {type, path: ['consoles', type, 'index'], title: `${cfg.name} (${type.toUpperCase()})`, props: {
        submissions: groupedConsoles[type],
      }}
    }),
    {type: 'cartridges', path: ['cartridges', 'index'], title: 'Game Boy cartridges', props: {
      games: R.sortBy(({game}) => game, (R.toPairs(groupedCartridges) as any[]).map(([type, submissions]) => {
        const cfg = config.gameCfgs[type];
        return {type, game: cfg.name, submissions}
      }))
    }},
  ];
  consoleSubmissions.forEach(submission => {
    const {type, slug, title, contributor} = submission;
    const cfg = config.consoleCfgs[type];
    pages.push({
      type: `${type}-console`,
      path: ['consoles', type, slug],
      title: `${type.toUpperCase()}: ${title} [${contributor}]`,
      props: {submission}
    });
  });
  cartridgeSubmissions.forEach(submission => {
    const {type, slug, title, contributor, game} = submission;
    const cfg = config.gameCfgs[type];
    pages.push({
      type: 'cartridge',
      path: ['cartridges', type, slug],
      title: `${cfg.name}: ${title} [${contributor}]`,
      props: {submission, cfg}
    });
  });
  R.forEachObjIndexed((submissions, type) => {
    const cfg = config.gameCfgs[type];
    pages.push({
      type: 'game',
      path: ['cartridges', type, 'index'],
      title: `${cfg.name}`,
      props: {type, cfg, submissions}
    })
  }, groupedCartridges);

  await Promise.all([
    Bluebird.map(pages, processPage, {concurrency: 16}),
    Bluebird.map(consoleSubmissions, processPhotos, {concurrency: 2}),
    Bluebird.map(cartridgeSubmissions, processPhotos, {concurrency: 2}),
  ]);

  await Promise.all([
    processConsoleCsv('dmg', DMG_CSV_COLUMNS, groupedConsoles.dmg),
    processConsoleCsv('sgb', SGB_CSV_COLUMNS, groupedConsoles.sgb),
    processConsoleCsv('mgb', MGB_CSV_COLUMNS, groupedConsoles.mgb),
    processConsoleCsv('mgl', MGL_CSV_COLUMNS, groupedConsoles.mgl),
    processConsoleCsv('sgb2', SGB2_CSV_COLUMNS, groupedConsoles.sgb2),
    processConsoleCsv('cgb', CGB_CSV_COLUMNS, groupedConsoles.cgb),
    processConsoleCsv('agb', AGB_CSV_COLUMNS, groupedConsoles.agb),
    processConsoleCsv('ags', AGS_CSV_COLUMNS, groupedConsoles.ags),
    processConsoleCsv('gbs', GBS_CSV_COLUMNS, groupedConsoles.gbs),
    processConsoleCsv('oxy', OXY_CSV_COLUMNS, groupedConsoles.oxy),
  ]);
  winston.info('Site generation finished :)');
}

async function processConsoleCsv<T, K extends keyof GroupedConsoleSubmissions>(
  key: K,
  columns: CsvColumn<T>[],
  rows: T[],
): Promise<void> {
  const dir = path.resolve('build', 'site', 'static', 'export', 'consoles');
  await fs.mkdirs(dir);
  return generateCsv(columns, rows, path.resolve(dir, `${key}.csv`))
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
