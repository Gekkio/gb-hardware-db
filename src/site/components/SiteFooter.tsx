import * as humanDate from 'human-date';
import * as React from 'react';

export default function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="site-footer__content">
        <License />
        <Stats />
      </div>
    </footer>
  )
}

function License() {
  return (
    <aside className="site-license">
      <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/" className="license__badge">
        <img className="site-license__image" alt="Creative Commons License" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"
             width="88" height="31" />
      </a>
      <p>
        The data and photos on this site are licensed under
        the <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons
      Attribution-ShareAlike 4.0 International License</a>.
      </p>
      <p>
        The <a href="https://github.com/Gekkio/gb-hardware-db">site source code</a> is licensed under the MIT license.
      </p>
    </aside>
  );
}

function Stats() {
  return (
    <aside className="site-stats">
      {`Last updated: ${humanDate.prettyPrint()}`}
      <br />
      <a href="/contribute/index.html">Want to contribute?</a>
    </aside>
  );
}
