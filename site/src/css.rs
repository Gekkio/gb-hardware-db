// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use std::path::Path;
use swc_common::BytePos;
use swc_css::{
    ast::Stylesheet,
    codegen::{
        writer::basic::{BasicCssWriter, BasicCssWriterConfig},
        CodeGenerator, CodegenConfig, Emit,
    },
    parser::{parse_str, parser::ParserConfig},
};

pub fn read_sass<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    let css = grass::from_path(
        path.to_str()
            .ok_or_else(|| anyhow!("Non-UTF8 path: {}", path.display()))?,
        &grass::Options::default(),
    )
    .map_err(|err| anyhow!("{}", err))?;
    Ok(css)
}

pub fn minify(css: &str) -> Result<String, Error> {
    let mut errors = Vec::new();
    let stylesheet: Stylesheet = parse_str(
        css,
        BytePos(0),
        BytePos(u32::try_from(css.len())?),
        ParserConfig::default(),
        &mut errors,
    )
    .map_err(|err| anyhow!("{:?}", err))?;
    let mut css = String::new();
    let mut gen = CodeGenerator::new(
        BasicCssWriter::new(&mut css, None, BasicCssWriterConfig::default()),
        CodegenConfig { minify: true },
    );
    gen.emit(&stylesheet)?;
    Ok(css)
}
