'use strict';

const gulp = require('gulp');
const postcss = require('gulp-postcss');
const sass = require('gulp-sass');
const sourcemaps = require('gulp-sourcemaps');
const ts = require('gulp-typescript');
const autoprefixer = require('autoprefixer');
const csso = require('postcss-csso');
const exec = require('child_process').exec;

const tsProject = ts.createProject('tsconfig.json')
gulp.task('scripts', function() {
  return tsProject.src()
    .pipe(sourcemaps.init())
    .pipe(tsProject())
    .js
    .pipe(sourcemaps.write())
    .pipe(gulp.dest('build'));
});

const postcssPlugins = [
  autoprefixer({browsers: ['last 2 versions', 'IE >= 10', 'iOS >= 7', '> 1%']}),
  csso()
];
gulp.task('styles', function () {
  return gulp.src('src/styles/**/*.scss')
    .pipe(sass().on('error', sass.logError))
    .pipe(postcss(postcssPlugins))
    .pipe(gulp.dest('build/site/static'));
});

gulp.task('html', ['scripts'], function(cb) {
  exec('node build/builder.js', function(err, stdout, stderr) {
    if (stdout) {
      console.log(stdout);
    }
    cb(err);
  });
});

gulp.task('build', ['html', 'scripts', 'styles']);
gulp.task('watch', ['html', 'scripts', 'styles'], function() {
  gulp.watch(['src/**/*.ts', 'src/**/*.tsx', 'data/**/*.json', 'data/**/*.jpg'], ['html']);
  gulp.watch('src/styles/**/*.scss', ['styles']);
});

gulp.task('default', ['build']);
