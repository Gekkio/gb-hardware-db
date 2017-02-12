import * as React from 'react';

namespace Home {
  export interface Props {
  }
}

class Home extends React.Component<Home.Props, {}> {
  render() {
    return (
      <article>
        <h2>Welcome to the Game Boy hardware database</h2>
      </article>
    )
  }
}

export default Home;
