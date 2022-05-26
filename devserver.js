// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

const express = require('express')

const app = express()
app.use(express.static('build'))
app.listen(8080, () => {
  console.info('Development server listening at port 8080')
})
