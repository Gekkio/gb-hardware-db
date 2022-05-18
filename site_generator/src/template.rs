use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::site::SiteSection;
use crate::template::{site_footer::SiteFooter, site_header::SiteHeader};

pub mod chip;
pub mod console_submission_list;
pub mod dmg_submission_list;
pub mod home;
pub mod markdown;
pub mod markdown_page;
pub mod raw_html;
pub mod site_footer;
pub mod site_header;

pub fn page(title: &str, section: SiteSection, content: VirtualNode) -> String {
    let content = html! {
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>{format!("{title} - Game Boy hardware database")}</title>
        <link rel="stylesheet" href="//fonts.googleapis.com/css?family=Lato:400,700" />
        <link rel="stylesheet" href="/static/gbhwdb.css" />
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <link rel="manifest" href="/site.webmanifest" />
        <link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5" />
        <meta name="msapplication-TileColor" content="#2b5797" />
        <meta name="theme-color" content="#ffffff" />
      </head>
      <body>
        <SiteHeader section={section} />
        <main class="site-main">
          <div class="site-main__content">{content}</div>
        </main>
        <SiteFooter />
      </body>
    </html>
    }
    .to_string();
    format!("<!DOCTYPE html>{content}")
}
