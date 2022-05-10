import 'source-map-support/register'
import Bluebird from 'bluebird'
import R from 'ramda'
import React from 'react'
import ReactDOMServer from 'react-dom/server'
import fs from 'fs-extra'
import * as path from 'path'
import * as process from 'process'
import * as winston from 'winston'

import Site from '../site/Site'
import { AgsSubmission, CartridgeSubmission, ConsoleSubmission, DmgSubmission, Photo } from '../crawler'
import * as config from '../config'
import { gameCfgs, gameLayouts, MapperId } from '../config'
import processPhotos from './processPhotos'
import util from 'util'

winston.configure({
  level: process.env.LOG_LEVEL || 'info',
  transports: [
    new winston.transports.Console({
      format: winston.format.simple(),
    }),
  ],
})

interface PageDeclaration {
  type: string
  path?: string[]
  title: string
  props: unknown
}

function getMapper({ type, metadata }: CartridgeSubmission): MapperId | undefined {
  if (metadata.board.mapper && metadata.board.mapper.kind) {
    switch (metadata.board.mapper.kind) {
      case 'MBC1':
      case 'MBC1A':
      case 'MBC1B':
      case 'MBC1B1':
        return 'mbc1'
      case 'MBC2':
      case 'MBC2A':
        return 'mbc2'
      case 'MBC3':
      case 'MBC3A':
      case 'MBC3B':
        return 'mbc3'
      case 'MBC30':
        return 'mbc30'
      case 'MBC5':
        return 'mbc5'
      case 'MBC6':
        return 'mbc6'
      case 'MBC7':
        return 'mbc7'
      case 'MMM01':
        return 'mmm01'
      case 'HuC-1':
      case 'HuC-1A':
        return 'huc1'
      case 'HuC-3':
        return 'huc3'
      case 'TAMA5':
        return 'tama5'
      default:
        console.warn(`Unsupported mapper type ${metadata.board.mapper.kind}`)
        return undefined
    }
  }
  const cfg = gameCfgs[type]
  const layout = cfg && gameLayouts[cfg.layouts[0]]
  if (!layout) return undefined
  return layout.chips.some(({ key }) => key === 'mapper') ? undefined : 'no-mapper'
}

async function crawlCartridges(): Promise<CartridgeSubmission[]> {
  const data = (await fs.readJson('build/data/cartridges.json')) as CartridgeSubmission[]
  return Bluebird.mapSeries(data, async (submission) => ({
    ...submission,
    photos: {
      front: await photoStats(submission.photos.front),
      pcbFront: await photoStats(submission.photos.pcbFront),
      pcbBack: await photoStats(submission.photos.pcbBack),
    },
  }))
}

async function crawlDmg(): Promise<DmgSubmission[]> {
  const data = (await fs.readJson('build/data/dmg.json')) as DmgSubmission[]
  return Bluebird.mapSeries(data, async (submission) => ({
    ...submission,
    photos: {
      front: await photoStats(submission.photos.front),
      back: await photoStats(submission.photos.back),
      mainboardFront: await photoStats(submission.photos.mainboardFront),
      mainboardBack: await photoStats(submission.photos.mainboardBack),
      lcdBoardFront: await photoStats(submission.photos.lcdBoardFront),
      lcdBoardBack: await photoStats(submission.photos.lcdBoardBack),
      powerBoardFront: await photoStats(submission.photos.powerBoardFront),
      powerBoardBack: await photoStats(submission.photos.powerBoardBack),
      jackBoardFront: await photoStats(submission.photos.jackBoardFront),
      jackBoardBack: await photoStats(submission.photos.jackBoardBack),
    },
  }))
}

async function crawlConsole(jsonFile: string): Promise<ConsoleSubmission[]> {
  const data = (await fs.readJson(jsonFile)) as Exclude<ConsoleSubmission, DmgSubmission | AgsSubmission>[]
  return Bluebird.mapSeries(data, async (submission) => ({
    ...submission,
    photos: {
      front: await photoStats(submission.photos.front),
      back: await photoStats(submission.photos.back),
      pcbFront: await photoStats(submission.photos.pcbFront),
      pcbBack: await photoStats(submission.photos.pcbBack),
    },
  }))
}

async function crawlAgs(): Promise<AgsSubmission[]> {
  const data = (await fs.readJson('build/data/ags.json')) as AgsSubmission[]
  return Bluebird.mapSeries(data, async (submission) => ({
    ...submission,
    photos: {
      front: await photoStats(submission.photos.front),
      top: await photoStats(submission.photos.top),
      back: await photoStats(submission.photos.back),
      pcbFront: await photoStats(submission.photos.pcbFront),
      pcbBack: await photoStats(submission.photos.pcbBack),
    },
  }))
}

async function photoStats(photo: Photo | undefined): Promise<Photo | undefined> {
  if (!photo) return undefined
  const stats = await fs.stat(photo.path)
  return {
    ...photo,
    stats,
  }
}

async function main(): Promise<void> {
  const cartridgeSubmissions = await crawlCartridges()
  const [
    dmgSubmissions,
    sgbSubmissions,
    mgbSubmissions,
    mglSubmissions,
    sgb2Submissions,
    cgbSubmissions,
    agbSubmissions,
    agsSubmissions,
    gbsSubmissions,
    oxySubmissions,
  ] = await Promise.all([
    crawlDmg(),
    crawlConsole('build/data/sgb.json'),
    crawlConsole('build/data/mgb.json'),
    crawlConsole('build/data/mgl.json'),
    crawlConsole('build/data/sgb2.json'),
    crawlConsole('build/data/cgb.json'),
    crawlConsole('build/data/agb.json'),
    crawlAgs(),
    crawlConsole('build/data/gbs.json'),
    crawlConsole('build/data/oxy.json'),
  ])
  const consoleSubmissions = sgbSubmissions
    .concat(dmgSubmissions)
    .concat(mgbSubmissions)
    .concat(mglSubmissions)
    .concat(sgb2Submissions)
    .concat(cgbSubmissions)
    .concat(agbSubmissions)
    .concat(agsSubmissions)
    .concat(gbsSubmissions)
    .concat(oxySubmissions)

  const cartridgesByGame: Record<string, CartridgeSubmission[]> = R.groupBy(({ type }) => type, cartridgeSubmissions)
  const cartridgesByMapper: Partial<Record<MapperId, CartridgeSubmission[]>> = {}

  for (const submission of cartridgeSubmissions) {
    const mapper = getMapper(submission)
    if (!mapper) continue
    const submissions = (cartridgesByMapper[mapper] = cartridgesByMapper[mapper] || [])
    submissions.push(submission)
  }

  const pages: PageDeclaration[] = [
    { type: 'consoles', path: ['consoles', 'index'], title: 'Game Boy consoles', props: {} },
    {
      type: 'cartridges',
      path: ['cartridges', 'index'],
      title: 'Game Boy cartridges',
      props: {
        games: R.sortBy(
          ({ cfg }) => cfg.name,
          Object.entries(cartridgesByGame).map(([type, submissions]) => {
            const cfg = config.gameCfgs[type]
            return { type, cfg, submissions }
          })
        ),
        mappers: Object.keys(cartridgesByMapper),
      },
    },
  ]
  consoleSubmissions.forEach((submission) => {
    const { type, slug, title, contributor } = submission
    pages.push({
      type: `${type}-console`,
      path: ['consoles', type, slug],
      title: `${type.toUpperCase()}: ${title} [${contributor}]`,
      props: { submission },
    })
  })
  cartridgeSubmissions.forEach((submission) => {
    const { type, slug, title, contributor } = submission
    const cfg = config.gameCfgs[type]
    pages.push({
      type: 'cartridge',
      path: ['cartridges', type, slug],
      title: `${cfg.name}: ${title} [${contributor}]`,
      props: { submission, cfg },
    })
  })
  R.forEachObjIndexed((submissions, type) => {
    const cfg = config.gameCfgs[type]
    pages.push({
      type: 'game',
      path: ['cartridges', type, 'index'],
      title: `${cfg.name}`,
      props: { type, cfg, submissions },
    })
  }, cartridgesByGame)
  R.forEachObjIndexed((submissions, mapper) => {
    pages.push({
      type: 'mapper',
      path: ['cartridges', mapper],
      title: `${mapper}`,
      props: { mapper, submissions },
    })
  }, cartridgesByMapper)

  await Promise.all([
    Bluebird.map(pages, processPage, { concurrency: 16 }),
    Bluebird.map(consoleSubmissions, processPhotos, { concurrency: 2 }),
    Bluebird.map(cartridgeSubmissions, processPhotos, { concurrency: 2 }),
  ])

  winston.info('Site generation finished :)')

  async function processPage(page: PageDeclaration): Promise<void> {
    const props = {
      pageType: page.type,
      title: `${page.title} - Game Boy hardware database`,
      pageProps: page.props,
      consoleSubmissionCount: consoleSubmissions.length,
      cartridgeSubmissionCount: cartridgeSubmissions.length,
    }
    const markup = ReactDOMServer.renderToStaticMarkup(React.createElement(Site, props))
    const html = `<!DOCTYPE html>\n${markup}`

    const directories = R.init(page.path || [])
    const targetDirectory = path.resolve('build', 'site', ...directories)

    const filename = R.last(page.path || []) || page.type
    const target = path.resolve(targetDirectory, `${filename}.html`)

    await fs.outputFile(target, html)
    winston.debug(`Wrote HTML file ${target}`)
  }
}

main()
  .then(() => null)
  .catch((e) => {
    console.error(util.types.isNativeError(e) ? e.stack : e)
    process.exit(1)
  })
