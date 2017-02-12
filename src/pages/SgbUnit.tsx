import * as React from 'react';

import {SgbSubmission} from '../crawler';

namespace SgbUnit {
  export interface Props {
    submission: SgbSubmission;
  }
}

class SgbUnit extends React.Component<SgbUnit.Props, {}> {
  renderMainPhoto() {
    const submission = this.props.submission;
    const {front} = this.props.submission.photos
    if (!front) {
      return null
    }
    return (
      <img src={`/static/sgb/${submission.slug}_${front.name}`} />
    )
  }
  render() {
    return (
      <article>
        <h2>{`SGB: ${this.props.submission.title}`}</h2>
        {this.renderMainPhoto()}
      </article>
    )
  }
}

export default SgbUnit;
