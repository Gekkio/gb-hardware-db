// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use percy_dom::{html, IterableNodes, View, VirtualNode};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SiteFooter;

impl View for SiteFooter {
    fn render(&self) -> VirtualNode {
        html! {
            <footer class="site-footer">
                <div class="site-footer__content">
                    <License />
                    <aside class="site-stats">
                        <a href="/contribute/index.html">{"Want to contribute?"}</a>
                    </aside>
                </div>
            </footer>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct License;

impl View for License {
    fn render(&self) -> VirtualNode {
        html! {
            <aside class="site-license">
                <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/" class="license__badge">
                <img
                  class="site-license__image"
                  alt="Creative Commons License"
                  src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"
                  width="88"
                  height="31">
                </a>
                <p>
                    {"The data and photos on this site are licensed under the "}
                    <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">
                        {"Creative Commons Attribution-ShareAlike 4.0 International License"}
                    </a>
                    .
                </p>
                <p>
                    {"The "}<a href="https://github.com/Gekkio/gb-hardware-db">{"site source code"}</a>{" is licensed under the MIT license."}
                </p>
            </aside>
        }
    }
}
