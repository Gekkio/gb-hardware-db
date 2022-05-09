use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::{
    legacy::{
        console::{ChipInfo, LegacyConsoleMetadata, LegacyConsolePhotos, LegacyMainboard},
        HasDateCode, LegacyPhoto, LegacySubmission,
    },
    template::chip::ConsoleListingChip,
};

#[derive(Copy, Clone, Debug)]
pub struct ConsoleSubmissionList<'a, M: LegacyConsoleMetadata, P: LegacyConsolePhotos> {
    pub submissions: &'a [LegacySubmission<M, P>],
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyConsolePhotos> View
    for ConsoleSubmissionList<'a, M, P>
{
    fn render(&self) -> VirtualNode {
        let console = M::CONSOLE;
        let chips = M::chips();
        html! {
            <article>
                <h2>{format!("{} ({})", console.name(), console.code())}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"ID"}</th>
                            <th>{"Board"}</th>
                            { chips.iter().map(|chip|
                                html! {
                                    <th>{format!("{} ({})", chip.label, chip.designator)}</th>
                                }
                            ).collect::<Vec<_>>() }
                            <th>{"Photos"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.submissions.iter().map(|submission|
                            Submission { submission, chips: &chips }.render()
                        ).collect::<Vec<_>>() }
                    </tbody>
                </table>
                <h3>{"Data dumps"}</h3>
                <a href={format!("/static/export/consoles/{id}.csv", id=console.id())}>{"UTF-8 encoded CSV"}</a>
            </article>
        }
    }
}

struct Submission<'a, M: LegacyConsoleMetadata, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub chips: &'a [ChipInfo<M>],
}

impl<'a, M: LegacyConsoleMetadata, P> Submission<'a, M, P> {
    fn photo(&self, label: &'static str, photo: Option<&LegacyPhoto>) -> Option<VirtualNode> {
        let console = M::CONSOLE;
        let slug = &self.submission.slug;
        photo.map(|photo| {
            let LegacyPhoto { name, .. } = photo;
            html! {
                <div>
                    <a href={format!("/static/{id}/{slug}_{name}", id=console.id())}>{label}</a>
                </div>
            }
        })
    }
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyConsolePhotos> View for Submission<'a, M, P> {
    fn render(&self) -> VirtualNode {
        let console = M::CONSOLE;
        let LegacySubmission {
            contributor,
            slug,
            title,
            metadata,
            photos,
            ..
        } = self.submission;
        html! {
            <tr>
                <td class="submission-list-item">
                    <a class="submission-list-item__link" href={format!("/consoles/{id}/{slug}.html", id=console.id())}>
                    <div class="submission-list-item__photo">
                    { P::photos().first().and_then(|photo| (photo.getter)(&photos)).map(|_| {
                        html! {
                            <img
                                src={format!("/static/{id}/{slug}_thumbnail_80.jpg", id=console.id())}
                                srcSet={format!("/static/{id}/{slug}_thumbnail_50.jpg 50w, /static/{id}/{slug}_thumbnail_80.jpg 80w", id=console.id())}
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
                        <div class="submission-list-item__title">{title}</div>
                        <aside class="submission-list-item__contributor">{contributor}</aside>
                    </div>
                    </a>
                </td>
                <td>
                    <div>{metadata.mainboard().kind()}</div>
                    <div>{metadata.mainboard().calendar_short()}</div>
                    {metadata.assembled().map(|date_code| {
                        html! {
                            <div>{format!("Assembled: {date_code}")}</div>
                        }
                    })}
                    {metadata.release_code().map(|release_code| {
                        html! {
                            <div>{format!("Release: {release_code}")}</div>
                        }
                    })}
                    {metadata.lcd_panel().map(|date_code| {
                        html! {
                            <div>{format!("LCD panel: {date_code}")}</div>
                        }
                    })}
                </td>
                { self.chips.iter().map(|chip| {
                    ConsoleListingChip {
                        chip: (chip.getter)(&metadata),
                        hide_type: chip.hide_type,
                    }.render()
                }).collect::<Vec<_>>() }
                <td>
                { P::photos().iter().filter_map(|photo| {
                    self.photo(photo.label, (photo.getter)(&photos))
                }).collect::<Vec<_>>() }
                </td>
            </tr>
        }
    }
}
