// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use image::{imageops, DynamicImage, ImageDecoder as _, RgbImage};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

pub mod cursive;
pub mod dat;

pub fn scale_to_px_limit(image: RgbImage, px_limit: u32) -> RgbImage {
    let int_scale_threshold = 70;

    if image.width() * image.height() > px_limit {
        let int_scale_size = (2..)
            .into_iter()
            .map(|divisor| (divisor, image.width() / divisor * image.height() / divisor))
            .filter(|&(_, pixels)| pixels <= px_limit)
            .next()
            .filter(|(_, pixels)| ((px_limit - pixels) * 100) / px_limit >= int_scale_threshold);
        let (target_width, target_height, image) = if let Some((divisor, _)) = int_scale_size {
            println!("Using integer scaling factor {divisor}");
            if image.width() % divisor > 0 || image.height() % divisor > 0 {
                let crop_width = image.width() / divisor * divisor;
                let crop_height = image.height() / divisor * divisor;
                println!(
                    "Cropping from {}x{} -> {}x{}",
                    image.width(),
                    image.height(),
                    crop_width,
                    crop_height,
                );
                (
                    image.width() / divisor,
                    image.height() / divisor,
                    imageops::crop_imm(
                        &image,
                        image.width() % divisor,
                        image.height() % divisor,
                        crop_width,
                        crop_height,
                    )
                    .to_image(),
                )
            } else {
                (image.width() / divisor, image.height() / divisor, image)
            }
        } else {
            let aspect_ratio = f64::from(image.width()) / f64::from(image.height());
            let height = (f64::from(px_limit) / aspect_ratio).sqrt().floor();
            let width = (height * aspect_ratio).round();
            (width as u32, height as u32, image)
        };
        println!(
            "Scaling from {}x{} -> {}x{}",
            image.width(),
            image.height(),
            target_width,
            target_height
        );
        imageops::resize(
            &image,
            target_width,
            target_height,
            imageops::FilterType::CatmullRom,
        )
    } else {
        image
    }
}

pub fn read_image(path: &Path) -> Result<RgbImage, Error> {
    let reader = image::io::Reader::new(BufReader::new(File::open(path)?));
    let mut decoder = reader.with_guessed_format()?.into_decoder()?;
    let icc = decoder
        .icc_profile()?
        .and_then(|data| qcms::Profile::new_from_slice(&data, false));
    let image = DynamicImage::from_decoder(decoder)?;
    let mut image = image.into_rgb8();

    if let Some(icc) = icc {
        println!("Convert to SRGB");
        let mut srgb = qcms::Profile::new_sRGB();
        srgb.precache_output_transform();

        let xfm = qcms::Transform::new(&icc, &srgb, qcms::DataType::RGB8, qcms::Intent::Perceptual)
            .unwrap();
        xfm.apply(&mut image);
    }
    Ok(image)
}

pub fn write_jpeg(image: &RgbImage, path: &Path) -> Result<(), Error> {
    let out_file = BufWriter::new(File::create(path)?);
    std::panic::catch_unwind(|| -> std::io::Result<()> {
        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
        comp.set_quality(75.0);
        comp.set_size(image.width() as usize, image.height() as usize);

        let mut comp = comp.start_compress(out_file)?;
        comp.write_scanlines(&image)?;
        comp.finish()?.flush()?;

        Ok(())
    })
    .map_err(|_| anyhow!("JPEG encoding failed"))??;

    Ok(())
}
