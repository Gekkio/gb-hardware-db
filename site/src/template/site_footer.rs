// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SiteFooter;

impl SiteFooter {
    fn license(&self) -> Markup {
        html! {
            aside.site-license {
                a.license__badge rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/" {
                    img.site-license__image
                      alt="Creative Commons License"
                      src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"
                      width="88"
                      height="31";
                }
                p {
                    "The data and photos on this site are licensed under the "
                    a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/" {
                        "Creative Commons Attribution-ShareAlike 4.0 International License"
                    }
                    "."
                }
                p {
                    "The "
                    a href="https://github.com/Gekkio/gb-hardware-db" { "site source code" }
                    " is licensed under the MIT license."
                }
            }
        }
    }
}

impl Render for SiteFooter {
    fn render(&self) -> Markup {
        html! {
            footer.site-footer {
                div.site-footer__content {
                    (self.license())
                    aside.site-stats {
                        a href="/contribute/index.html" { "Want to contribute?" }
                    }
                }
            }
        }
    }
}
