// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{Markup, Render, html};

use crate::legacy::{LegacyPhoto, LegacyPhotos, LegacySubmission};

pub struct ListingPhotosCell<'a, M, P> {
    pub submission: &'a LegacySubmission<M, P>,
}

impl<'a, M, P> ListingPhotosCell<'a, M, P> {
    fn render_photo(&self, label: &'static str, photo: &LegacyPhoto) -> Markup {
        html! {
            div {
                a href={ "/static/" (self.submission.code) "/" (self.submission.slug) "_" (photo.name) } {
                    (label)
                }
            }
        }
    }
}

impl<'a, M, P> Render for ListingPhotosCell<'a, M, P>
where
    P: LegacyPhotos,
{
    fn render(&self) -> Markup {
        html! {
            td {
                @for info in P::infos() {
                    @if let Some(photo) = (info.getter)(&self.submission.photos) {
                        (self.render_photo(info.label, photo))
                    }
                }
            }
        }
    }
}
