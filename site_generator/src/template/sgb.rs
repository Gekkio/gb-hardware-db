use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::{
    legacy::{HasDateCode, LegacyPhoto, LegacySgbSubmission, LegacySubmission},
    template::chip::ConsoleListingChip,
};

#[derive(Copy, Clone, Debug)]
pub struct Sgb<'a> {
    pub submissions: &'a [LegacySgbSubmission],
}

impl<'a> View for Sgb<'a> {
    fn render(&self) -> VirtualNode {
        html! {
            <article>
                <h2>{"Super Game Boy (SGB)"}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"ID"}</th>
                            <th>{"Board"}</th>
                            <th>{"CPU (U1)"}</th>
                            <th>{"ICD2 (U2)"}</th>
                            <th>{"WRAM (U3)"}</th>
                            <th>{"VRAM (U4)"}</th>
                            <th>{"ROM (U5)"}</th>
                            <th>{"CIC (U6)"}</th>
                            <th>{"Photos"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.submissions.iter().map(|submission|
                            Submission { submission }.render()
                        ).collect::<Vec<_>>() }
                    </tbody>
                </table>
                <h3>{"Data dumps"}</h3>
                <a href="/static/export/consoles/sgb.csv">{"UTF-8 encoded CSV"}</a>
            </article>
        }
    }
}

struct Submission<'a> {
    pub submission: &'a LegacySgbSubmission,
}

impl<'a> View for Submission<'a> {
    fn render(&self) -> VirtualNode {
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
                    <a class="submission-list-item__link" href={format!("/consoles/sgb/{slug}.html")}>
                    <div class="submission-list-item__photo">
                    { photos.front.as_ref().map(|_| {
                        html! {
                            <img
                                src={format!("/static/sgb/{slug}_thumbnail_80.jpg")}
                                srcSet={format!("/static/sgb/{slug}_thumbnail_50.jpg 50w, /static/sgb/{slug}_thumbnail_80.jpg 80w")}
                                sizes="(min-width: 1000px) 80px, 50px"
                                role="presentation"
                            >
                        }
                    }) }
                    </div>
                    <div class="submission-list-item__id">
                        <div class="submission-list-item__title">{title}</div>
                        <aside class="submission-list-item__contributor">{contributor}</aside>
                    </div>
                    </a>
                </td>
                <td>
                    <div>{&metadata.mainboard.kind}</div>
                    <div>{metadata.mainboard.calendar_short()}</div>
                </td>
                { ConsoleListingChip::new(metadata.mainboard.cpu.as_ref()).render() }
                { ConsoleListingChip::new(metadata.mainboard.icd2.as_ref()).render() }
                { ConsoleListingChip::new(metadata.mainboard.work_ram.as_ref()).render() }
                { ConsoleListingChip::new(metadata.mainboard.video_ram.as_ref()).render() }
                { ConsoleListingChip::new(metadata.mainboard.rom.as_ref()).render() }
                { ConsoleListingChip::new(metadata.mainboard.cic.as_ref()).render() }
                <td>
                    { photo(slug, "Front", &photos.front) }
                    { photo(slug, "Back", &photos.back) }
                    { photo(slug, "PCB front", &photos.pcb_front) }
                    { photo(slug, "PCB back", &photos.pcb_back) }
                </td>
            </tr>
        }
    }
}

fn photo(slug: &str, label: &'static str, photo: &Option<LegacyPhoto>) -> Option<VirtualNode> {
    photo.as_ref().map(|photo| {
        let LegacyPhoto { name, .. } = photo;
        html! {
            <div>
                <a href={format!("/static/sgb/{slug}_{name}")}>{label}</a>
            </div>
        }
    })
}
