# SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
#
# SPDX-License-Identifier: CC0-1.0

dev:
  just run-devserver &
  cargo watch -- just build-site

run-devserver:
  cargo run --bin gbhwdb-devserver

build-site:
  cargo run --bin gbhwdb-site
