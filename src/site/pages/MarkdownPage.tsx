import React from 'react'
import ReactMarkdown from 'react-markdown'

interface Props {
  content: string
}

export default function MarkdownPage({ content }: Props) {
  return (
    <article>
      <ReactMarkdown>{content}</ReactMarkdown>
    </article>
  )
}
