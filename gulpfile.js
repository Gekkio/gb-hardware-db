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
    .pipe(gulp.dest('build/scripts'));
});

const postcssPlugins = [
  autoprefixer({browsers: ['last 2 versions', 'IE >= 10', 'iOS >= 7', '> 1%']}),
  csso()
];
gulp.task('styles', function() {
  return gulp.src('src/site/**/*.scss')
    .pipe(sass().on('error', sass.logError))
    .pipe(postcss(postcssPlugins))
    .pipe(gulp.dest('build/site/static'));
});

gulp.task('static', function() {
  return gulp.src('static/**/*.html')
    .pipe(gulp.dest('build/site'));
})

gulp.task('html', ['scripts'], function(cb) {
  exec('node build/scripts/builder/builder.js', function(err, stdout, stderr) {
    if (stdout) {
      console.log(stdout);
    }
    if (stderr) {
      console.error(stderr);
    }
    cb(err);
  });
});

gulp.task('build', ['html', 'scripts', 'styles', 'static']);
gulp.task('watch', ['html', 'scripts', 'styles'], function() {
  gulp.watch(['src/**/*.ts', 'src/**/*.tsx', 'data/**/*.json', 'data/**/*.jpg'], ['html']);
  gulp.watch('src/site/**/*.scss', ['styles']);
  gulp.watch('static/**/*.html', ['static']);
});

gulp.task('default', ['build']);
