# SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
#
# SPDX-License-Identifier: CC0-1.0

dev:
  (cargo run --bin gbhwdb-devserver) & (cargo watch -- cargo run --bin gbhwdb-site) & wait
