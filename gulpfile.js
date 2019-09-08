'use strict'

const gulp = require('gulp')
const postcss = require('gulp-postcss')
const sass = require('gulp-sass')
const sourcemaps = require('gulp-sourcemaps')
const eslint = require('gulp-eslint')
const ts = require('gulp-typescript')
const process = require('process')
const exec = require('child_process').exec

const staticPaths = [
  'static/**/*.html',
  'static/**/*.txt',
  'static/**/*.ico',
  'static/**/*.png',
  'static/**/*.svg',
  'static/**/*.webmanifest',
  'static/**/*.xml',
]

const tsProject = ts.createProject('tsconfig.json')
const scripts = () =>
  tsProject
    .src()
    .pipe(sourcemaps.init())
    .pipe(tsProject())
    .js.pipe(sourcemaps.write())
    .pipe(gulp.dest('build/scripts'))

const lintProject = ts.createProject('tsconfig.json')
const lint = () =>
  lintProject
    .src()
    .pipe(eslint())
    .pipe(eslint.format())
    .pipe(eslint.failAfterError())

const styles = () =>
  gulp
    .src('src/site/**/*.scss')
    .pipe(sass().on('error', sass.logError))
    .pipe(postcss())
    .pipe(gulp.dest('build/site/static'))

const staticFiles = () => gulp.src(staticPaths).pipe(gulp.dest('build/site'))

const html = done => {
  const node = exec('node build/scripts/builder/builder.js', done)
  node.stdout.pipe(process.stdout)
  node.stderr.pipe(process.stderr)
}
const scriptsAndHtml = gulp.series(scripts, html)

const build = gulp.parallel(scriptsAndHtml, styles, staticFiles)
const watch = async () => {
  gulp.watch(['src/**/*.ts', 'src/**/*.tsx'], scripts)
  gulp.watch(['build/scripts/**/*.js', 'content/**/*.markdown', 'data/**/*.json', 'data/**/*.jpg'], html)
  gulp.watch('src/site/**/*.scss', styles)
  gulp.watch(staticPaths, staticFiles)
}

module.exports = {
  html,
  styles,
  scripts,
  static: staticFiles,
  build,
  watch,
  lint,
  default: build,
}
