// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::legacy::{LegacyMetadata, LegacyPhotos, LegacySubmission};

pub struct ListingEntryCell<'a, M, P> {
    pub url_prefix: &'static str,
    pub primary_text: &'a str,
    pub secondary_texts: &'a [&'a str],
    pub submission: &'a LegacySubmission<M, P>,
}

impl<'a, M, P> Render for ListingEntryCell<'a, M, P>
where
    M: LegacyMetadata,
    P: LegacyPhotos,
{
    fn render(&self) -> Markup {
        let url_prefix = self.url_prefix;
        let code = &self.submission.code;
        let slug = &self.submission.slug;
        let first_photo = P::infos()
            .first()
            .and_then(|photo| (photo.getter)(&self.submission.photos));
        html! {
            td.listing-entry-cell {
                a.listing-entry-cell__link href=(format!("{url_prefix}/{code}/{slug}.html")) {
                    div.listing-entry-cell__photo {
                        @if first_photo.is_some() {
                            img
                                src=(format!("/static/{code}/{slug}_thumbnail_80.jpg"))
                                srcSet=(format!("/static/{code}/{slug}_thumbnail_50.jpg 50w, /static/{code}/{slug}_thumbnail_80.jpg 80w"))
                                sizes="(min-width: 1000px) 80px, 50px"
                                role="presentation";
                        } @else if let Some(src) = M::PLACEHOLDER_SVG {
                            img.listing-entry-cell__placeholder src=(src) role="presentation";
                        }
                    }
                    div.listing-entry-cell__text {
                        div.listing-entry-cell__primary { (self.primary_text) }
                        @for text in self.secondary_texts {
                            aside { (text) }
                        }
                        aside.listing-entry-cell__contributor { (self.submission.contributor) }
                    }
                }
            }
        }
    }
}
