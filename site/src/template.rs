// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::{
    site::SiteSection,
    template::{site_footer::SiteFooter, site_header::SiteHeader},
};

pub mod cartridge_page;
pub mod cartridges;
pub mod console_page;
pub mod console_submission_list;
pub mod contributor_cartridges;
pub mod dmg_console_page;
pub mod dmg_submission_list;
pub mod game;
pub mod home;
pub mod listing_entry_cell;
pub mod listing_part;
pub mod listing_photos_cell;
pub mod mapper;
pub mod markdown;
pub mod markdown_page;
pub mod platform_cartridges;
pub mod site_footer;
pub mod site_header;
pub mod submission_part_table;

pub fn page(title: &str, section: SiteSection, content: Markup) -> String {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title {
                    (title) " - Game Boy hardware database"
                }
                link rel="stylesheet" href="//fonts.googleapis.com/css?family=Lato:400,700";
                link rel="stylesheet" href="/static/gbhwdb.css";
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
                link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5";
                meta name="msapplication-TileColor" content="#2b5797";
                meta name="theme-color" content="#ffffff";
            }
            body {
                (SiteHeader { section })
                main.site-main {
                    div.site-main__content { (content) }
                }
                (SiteFooter)
                script {
                    (PreEscaped("document.querySelectorAll('.jsonly').forEach((m) => { m.hidden = false; });"))
                }
            }
        }
    }
    .into_string()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Optional<T>(Option<T>);

impl<T> Render for Optional<T>
where
    T: Render,
{
    fn render_to(&self, buffer: &mut String) {
        if let Some(value) = &self.0 {
            value.render_to(buffer);
        }
    }
}
