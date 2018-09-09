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
  AgbSubmission, AgsSubmission, CartridgeSubmission, CgbSubmission, ConsoleSubmission, crawlCartridges, crawlConsoles,
  DmgSubmission, GbsSubmission, MgbSubmission, MglSubmission, OxySubmission, Sgb2Submission, SgbSubmission
} from '../crawler';
import {
  AGB_CSV_COLUMNS, AGS_CSV_COLUMNS, CARTRIDGE_CSV_COLUMNS, CGB_CSV_COLUMNS, CsvColumn, DMG_CSV_COLUMNS, GBS_CSV_COLUMNS,
  generateCsv, MGB_CSV_COLUMNS, MGL_CSV_COLUMNS, OXY_CSV_COLUMNS, SGB2_CSV_COLUMNS, SGB_CSV_COLUMNS
} from './csvTransform';
import * as config from '../config';
import {ConsoleType, gameCfgs, gameLayouts, MapperId} from '../config';
import processPhotos from './processPhotos';

winston.configure({
  level: process.env.LOG_LEVEL || 'info',
  transports: [
    new winston.transports.Console({
      format: winston.format.simple()
    }),
  ]
});

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

function getMapper({type, metadata}: CartridgeSubmission): MapperId | undefined {
  if (metadata.board.mapper && metadata.board.mapper.type) {
    switch (metadata.board.mapper.type) {
      case 'MBC1':
      case 'MBC1A':
      case 'MBC1B':
      case 'MBC1B1':
        return 'mbc1';
      case 'MBC2':
      case 'MBC2A':
        return 'mbc2';
      case 'MBC3':
      case 'MBC3A':
      case 'MBC3B':
        return 'mbc3';
      case 'MBC30':
        return 'mbc30';
      case 'MBC5':
        return 'mbc5';
      case 'MBC6':
        return 'mbc6';
      case 'MBC7':
        return 'mbc7';
      case 'MMM01':
        return 'mmm01';
      case 'HuC-1':
      case 'HuC-1A':
        return 'huc1';
      case 'HuC-3':
        return 'huc3';
      case 'TAMA5':
        return 'tama5';
      default:
        console.warn(`Unsupported mapper type ${metadata.board.mapper.type}`)
        return undefined
    }
  }
  const cfg = gameCfgs[type];
  const layout = cfg && gameLayouts[cfg.layout]
  if (!layout) return undefined;
  return (layout.chips.some(({key}) => key === 'mapper'))
    ? undefined
    : 'no-mapper'
}

function sortGroupComparator(a: ConsoleSubmission, b: ConsoleSubmission): number {
  if (a.sortGroup) {
    return b.sortGroup ? a.sortGroup.localeCompare(b.sortGroup) : -1
  } else {
    return b.sortGroup ? 1 : 0
  }
}
const slugComparator = R.comparator((a: ConsoleSubmission, b: ConsoleSubmission) => a.slug < b.slug);

function consoleSubmissionComparator(a: ConsoleSubmission, b: ConsoleSubmission): number {
  return sortGroupComparator(a, b) || slugComparator(a, b)
}

async function main(): Promise<void> {
  const [consoleSubmissions, cartridgeSubmissions] = await Promise.all([
    crawlConsoles('data/consoles'),
    crawlCartridges('data/cartridges'),
  ]);

  const groupedConsoles: GroupedConsoleSubmissions = R.map(
    R.sort(consoleSubmissionComparator),
    R.groupBy(({type}) => type, consoleSubmissions) as Record<ConsoleType, ConsoleSubmission[]>) as any;
  const cartridgesByGame: Record<string, CartridgeSubmission[]> = R.groupBy(({type}) => type, cartridgeSubmissions);
  const cartridgesByMapper: Partial<Record<MapperId, CartridgeSubmission[]>> = {};

  for (const submission of cartridgeSubmissions) {
    const mapper = getMapper(submission);
    if (!mapper) continue;
    const submissions = cartridgesByMapper[mapper] = cartridgesByMapper[mapper] || [];
    submissions.push(submission)
  }

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
      games: R.sortBy(({game}) => game, (R.toPairs(cartridgesByGame) as any[]).map(([type, submissions]) => {
        const cfg = config.gameCfgs[type];
        return {type, game: cfg.name, submissions}
      })),
      mappers: Object.keys(cartridgesByMapper),
    }},
  ];
  consoleSubmissions.forEach(submission => {
    const {type, slug, title, contributor} = submission;
    const cfg = config.consoleCfgs[type];
    pages.push({
      type: `${type}-console`,
      path: ['consoles', type, slug],
      title: `${type.toUpperCase()}: ${title} [${contributor}]`,
      props: {submission},
    });
  });
  cartridgeSubmissions.forEach(submission => {
    const {type, slug, title, contributor} = submission;
    const cfg = config.gameCfgs[type];
    pages.push({
      type: 'cartridge',
      path: ['cartridges', type, slug],
      title: `${cfg.name}: ${title} [${contributor}]`,
      props: {submission, cfg},
    });
  });
  R.forEachObjIndexed((submissions, type) => {
    const cfg = config.gameCfgs[type];
    pages.push({
      type: 'game',
      path: ['cartridges', type, 'index'],
      title: `${cfg.name}`,
      props: {type, cfg, submissions},
    })
  }, cartridgesByGame);
  R.forEachObjIndexed((submissions, mapper) => {
    pages.push({
      type: 'mapper',
      path: ['cartridges', mapper],
      title: `${mapper}`,
      props: {mapper, submissions},
    })
  }, cartridgesByMapper);

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
    processCartridgeCsv(cartridgeSubmissions),
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

async function processCartridgeCsv(submissions: CartridgeSubmission[]): Promise<void> {
  const dir = path.resolve('build', 'site', 'static', 'export');
  await fs.mkdirs(dir);
  return generateCsv(CARTRIDGE_CSV_COLUMNS, submissions, path.resolve(dir, `cartridges.csv`))
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
