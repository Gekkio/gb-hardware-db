// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{LegacyMetadata, LegacyPhotos, LegacySubmission};

pub struct ListingEntryCell<'a, M, P> {
    pub url_prefix: &'static str,
    pub primary_text: &'a str,
    pub secondary_texts: &'a [&'a str],
    pub submission: &'a LegacySubmission<M, P>,
}

impl<'a, M, P> View for ListingEntryCell<'a, M, P>
where
    M: LegacyMetadata,
    P: LegacyPhotos,
{
    fn render(&self) -> VirtualNode {
        let url_prefix = self.url_prefix;
        let code = &self.submission.code;
        let slug = &self.submission.slug;
        html! {
            <td class="listing-entry-cell">
                <a class="listing-entry-cell__link" href={format!("{url_prefix}/{code}/{slug}.html")}>
                    <div class="listing-entry-cell__photo">
                    { P::infos().first().and_then(|photo| (photo.getter)(&self.submission.photos)).map(|_| {
                        html! {
                            <img
                                src={format!("/static/{code}/{slug}_thumbnail_80.jpg")}
                                srcSet={format!("/static/{code}/{slug}_thumbnail_50.jpg 50w, /static/{code}/{slug}_thumbnail_80.jpg 80w")}
                                sizes="(min-width: 1000px) 80px, 50px"
                                role="presentation"
                            >
                        }
                    }).or_else(|| {
                        M::PLACEHOLDER_SVG.map(|src| {
                            html! {
                                <img src={src} class="listing-entry-cell__placeholder" role="presentation">
                            }
                        })
                    }) }
                    </div>
                    <div class="listing-entry-cell__text">
                        <div class="listing-entry-cell__primary">{self.primary_text}</div>
                        {self.secondary_texts.iter().map(|&text| html! {
                            <aside>{text}</aside>
                        }).collect::<Vec<_>>() }
                        <aside class="listing-entry-cell__contributor">{&self.submission.contributor}</aside>
                    </div>
                </a>
            </td>
        }
    }
}
