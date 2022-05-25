use percy_dom::{html, IterableNodes, VirtualNode};

use crate::legacy::{LegacyMetadata, LegacyPhoto, LegacyPhotos, LegacySubmission};

pub fn submission_list_submission<M: LegacyMetadata, P: LegacyPhotos>(
    url_prefix: &'static str,
    submission: &LegacySubmission<M, P>,
) -> VirtualNode {
    let code = &submission.code;
    let slug = &submission.slug;
    html! {
        <td class="submission-list-item">
            <a class="submission-list-item__link" href={format!("{url_prefix}/{code}/{slug}.html")}>
            <div class="submission-list-item__photo">
            { P::infos().first().and_then(|photo| (photo.getter)(&submission.photos)).map(|_| {
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
                        <img src={src} class="submission-list-item__placeholder" role="presentation">
                    }
                })
            }) }
            </div>
            <div class="submission-list-item__id">
                <div class="submission-list-item__title">{&submission.title}</div>
                <aside class="submission-list-item__contributor">{&submission.contributor}</aside>
            </div>
            </a>
        </td>
    }
}

pub fn submission_list_photos<M, P: LegacyPhotos>(
    submission: &LegacySubmission<M, P>,
) -> VirtualNode {
    html! {
        <td>
            { P::infos().iter().filter_map(|photo| {
                render_photo(submission, photo.label, (photo.getter)(&submission.photos))
            }).collect::<Vec<_>>() }
        </td>
    }
}

fn render_photo<M, P>(
    submission: &LegacySubmission<M, P>,
    label: &'static str,
    photo: Option<&LegacyPhoto>,
) -> Option<VirtualNode> {
    let code = &submission.code;
    let slug = &submission.slug;
    photo.map(|photo| {
        let name = &photo.name;
        html! {
            <div>
                <a href={format!("/static/{code}/{slug}_{name}")}>{label}</a>
            </div>
        }
    })
}
