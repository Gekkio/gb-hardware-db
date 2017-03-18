import * as React from 'react';
import * as ReactMarkdown from 'react-markdown';

interface Props {
  content: string;
}

export default function Home({content}: Props) {
  return (
    <ReactMarkdown source={content} containerTagName="article" />
  )
}
