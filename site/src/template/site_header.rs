// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::Console;
use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::site::SiteSection;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SiteHeader {
    pub section: SiteSection,
}

impl View for SiteHeader {
    fn render(&self) -> VirtualNode {
        let consoles_class = match self.section {
            SiteSection::Consoles(_) => "active",
            SiteSection::Cartridges => "",
        };
        let cartridges_class = match self.section {
            SiteSection::Consoles(_) => "",
            SiteSection::Cartridges => "active",
        };
        html! {
            <header class="site-header">
                <div class="site-header__primary">
                    <h1 class="site-header__title">
                        <a href="/">
                            {"Game Boy hardware database"}
                            <aside>{"by Gekkio and contributors"}</aside>
                        </a>
                    </h1>
                    <nav class="site-primary-nav">
                        <ul>
                            <li class={consoles_class}>
                                <a href="/">{"Consoles"}</a>
                            </li>
                            <li class={cartridges_class}>
                                <a href="/cartridges">{"Game cartridges"}</a>
                            </li>
                        </ul>
                    </nav>
                </div>
                <SecondaryNav section={self.section} />
            </header>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct SecondaryNav {
    pub section: SiteSection,
}

impl View for SecondaryNav {
    fn render(&self) -> VirtualNode {
        match self.section {
            SiteSection::Consoles(selected) => {
                html! {
                    <nav class="site-secondary-nav">
                        <ul>
                            { Console::ALL.iter().map(|&console| {
                                let class = if Some(console) == selected { "active" } else { "" };
                                html! {
                                    <li class={class}>
                                        <a href={format!("/consoles/{}", console.id())}>
                                            <strong>{console.code()}</strong>
                                            <span class="name">{console.name()}</span>
                                        </a>
                                    </li>
                                }
                            }).collect::<Vec<_>>() }
                        </ul>
                    </nav>
                }
            }
            SiteSection::Cartridges => html! {
                <nav class="site-secondary-nav" />
            },
        }
    }
}
