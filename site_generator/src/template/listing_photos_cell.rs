use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{LegacyPhoto, LegacyPhotos, LegacySubmission};

pub struct ListingPhotosCell<'a, M, P> {
    pub submission: &'a LegacySubmission<M, P>,
}

impl<'a, M, P> ListingPhotosCell<'a, M, P> {
    fn render_photo(&self, label: &'static str, photo: &LegacyPhoto) -> VirtualNode {
        let code = &self.submission.code;
        let slug = &self.submission.slug;
        let name = &photo.name;
        html! {
            <div>
                <a href={format!("/static/{code}/{slug}_{name}")}>{label}</a>
            </div>
        }
    }
}

impl<'a, M, P> View for ListingPhotosCell<'a, M, P>
where
    P: LegacyPhotos,
{
    fn render(&self) -> VirtualNode {
        html! {
            <td>
                { P::infos().iter().filter_map(|info| {
                    (info.getter)(&self.submission.photos).map(|photo| self.render_photo(info.label, photo))
                }).collect::<Vec<_>>() }
            </td>
        }
    }
}
