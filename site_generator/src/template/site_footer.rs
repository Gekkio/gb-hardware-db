use percy_dom::{html, IterableNodes, View, VirtualNode};
use time::{macros::format_description, Date};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SiteFooter {
    pub today: Date,
    pub console_submission_count: u32,
    pub cartridge_submission_count: u32,
}

impl View for SiteFooter {
    fn render(&self) -> VirtualNode {
        let today = self
            .today
            .format(format_description!(
                "[month repr:long] [day padding:none], [year]"
            ))
            .unwrap_or_else(|_| "?".to_string());
        let console_submission_count = self.console_submission_count;
        let cartridge_submission_count = self.cartridge_submission_count;
        html! {
            <footer class="site-footer">
                <div class="site-footer__content">
                    <License />
                    <aside class="site-stats">
                        {format!("Last updated: {today}")}
                        <br>
                        {format!("Console submission count: {console_submission_count}")}
                        <br>
                        {format!("Cartridge submission count: {cartridge_submission_count}")}
                        <br>
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
